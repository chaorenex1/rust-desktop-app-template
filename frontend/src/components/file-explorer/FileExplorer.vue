<script setup lang="ts">
import { Folder, Document, Plus, Refresh, Search } from '@element-plus/icons-vue';
import { ElTree, ElInput, ElButton, ElIcon } from 'element-plus';
import { ref, onMounted, watch } from 'vue';
import { useFileStore } from '@/stores/filesStore';
import { useAppStore } from '@/stores/appStore';
import { showError } from '@/utils/toast';

import {
  listFiles,
  readFile,
  writeFile,
  createDirectory
} from '@/services/tauri/commands';
import { f } from 'vue-router/dist/router-CWoNjPRp.mjs';
import { da } from 'element-plus/es/locales.mjs';
import type { FileItem } from '@/utils/types';

const fileStore = useFileStore();
const appStore = useAppStore();

const searchQuery = ref('');
const treeRef = ref<InstanceType<typeof ElTree> | null>(null);

interface FileNode {
  name: string;
  path: string;
  isDirectory: boolean;
  size?: number;
  modified?: string;
  isLeaf?: boolean;
}

const treeData = ref<FileNode[]>([]);
const rootDirectory = ref<FileNode | null>(null);
const isLoading = ref(false);


const defaultProps = {
  children: 'children',
  label: 'name',
  isLeaf: 'isLeaf',
};

// 初始化根目录列表
onMounted(async () => {
  // 如果已经在其他地方加载过目录（例如最近目录或设置面板），直接使用现有数据
  await initialize();
});

// 根据文件列表变化更新根节点
watch(
  () => fileStore.files,
  (files) => {
    treeData.value = files.map((file) => ({
      name: file.name,
      path: file.path,
      isDirectory: file.isDirectory,
      size: file.size,
      modified: file.modified,
      isLeaf: !file.isDirectory,
    }));
  },
  { deep: true }
);

// 监听根目录变化，重新初始化
watch(
  () => fileStore.currentDirectory,
  async (newDir, oldDir) => {
    if (newDir !== oldDir) {
      console.debug('Root directory changed, reinitializing...', { newDir, oldDir });
      await initialize(true);
    }
  }
);

async function initialize(refresh: boolean = false) {
  if (!fileStore.files.length || refresh) {
    await fileStore.loadDirectory(fileStore.getRootDirectory());
  }
  rootDirectory.value = {
    name: fileStore.currentDirectory,
    path: fileStore.currentDirectory,
    isDirectory: true,
    isLeaf: false,
  };

  console.debug('Root directory list:', fileStore.files);

  treeData.value = fileStore.files.map((file) => ({
    name: file.name,
    path: file.path,
    isDirectory: file.isDirectory,
    size: file.size,
    modified: file.modified,
    isLeaf: !file.isDirectory,
  }));
}


// 懒加载子目录（根节点完全由 treeData 提供）
async function loadNode(node: any, resolve: (data: FileNode[]) => void) {
  const data = node.data as FileNode | undefined;
  
  // Element Plus 初始化时可能调用 loadNode，此时 node.data 为 undefined
  // 根节点数据由 :data="treeData" 提供，无需在这里加载，直接返回即可
  if (!data) {
    return resolve([]);
  }

  // 非目录节点（文件）不需要加载子节点，也不触发任何日志
  if (!data.isDirectory) {
    return resolve([]);
  }

  // 只有目录节点才会执行到这里
  console.debug('Loading directory node:', { 
    nodePath: data.path, 
    level: node.level 
  });

  try {
    const fileList = await listFiles(data.path);
    console.debug(`Loaded ${fileList.length} items from ${data.path}`);

    const children: FileNode[] = fileList.map((file) => ({
      name: file.name,
      path: file.path,
      isDirectory: file.isDirectory,
      size: file.size,
      modified: file.modified,
      isLeaf: !file.isDirectory,
    }));
    
    console.debug('Children nodes:', children.map(c => ({ 
      name: c.name, 
      isDirectory: c.isDirectory,
      isLeaf: c.isLeaf 
    })));

    resolve(children);
  } catch (error) {
    console.error('加载子目录失败', error);
    resolve([]);
  }
}

// 过滤节点（搜索）
function filterNode(value: string, data: any) {
  if (!value) return true;
  return data.name.toLowerCase().includes(value.toLowerCase());
}

watch(searchQuery, (val) => {
  treeRef.value?.filter(val);
});

// 处理节点点击：目录展开/收起，文件在编辑器打开
async function handleNodeClick(data: FileNode, node: any) {
  if (data.isDirectory) {
    node.expanded = !node.expanded;
  } else {
    try {
      // 显示加载状态
      if (!isLoading.value) {
        isLoading.value = true;
        await fileStore.openFile(data.path);
      }else{
        showWarning('文件正在打开中，请稍后再试');
      }
    } catch (error) {
      showError(
        (error instanceof Error && error.message) || '打开文件失败（可能不是文本文件或编码不兼容）',
        '打开文件失败'
      );
    } finally {
      // 确保加载状态被清除
      isLoading.value = false;
    }
  }
}

// Create new file/folder
async function createNew(isDirectory = false) {
  const name = prompt(`请输入${isDirectory ? '文件夹' : '文件'}名称:`);
  if (name) {
    await fileStore.createFile(name, isDirectory);
  }
}

// Refresh directory
async function refreshDirectory() {
  await initialize(true);
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Toolbar -->
    <div class="border-b border-border bg-surface p-2">
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center space-x-1">
          <ElButton
            :icon="Plus"
            size="small"
            text
            @click="createNew(false)"
          >
            文件
          </ElButton>
          <ElButton
            :icon="Folder"
            size="small"
            text
            @click="createNew(true)"
          >
            文件夹
          </ElButton>
        </div>
        <ElButton
          :icon="Refresh"
          size="small"
          text
          @click="refreshDirectory"
        />
      </div>

      <ElInput
        v-model="searchQuery"
        :prefix-icon="Search"
        placeholder="搜索文件..."
        size="small"
        clearable
      />
    </div>

    <!-- File Tree -->
    <div class="flex-1 overflow-auto p-2">
      <div
        v-if="fileStore.isLoading"
        class="flex flex-col items-center justify-center h-full"
      >
        <div
          class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-primary-500"
        />
        <p class="mt-2 text-sm text-text-secondary">
          加载中...
        </p>
      </div>

      <div
        v-else-if="fileStore.error && !treeData.length"
        class="text-center p-4 text-error"
      >
        {{ fileStore.error }}
      </div>

      <ElTree
        v-else
        ref="treeRef"
        :data="treeData"
        :props="defaultProps"
        node-key="path"
        lazy
        highlight-current
        :load="loadNode"
        :filter-node-method="filterNode"
        @node-click="handleNodeClick"
        :indent="20"
      >
        <template #default="{ data }">
          <div class="flex items-center px-1 py-1">
            <ElIcon
              :size="16"
              class="mr-2"
            >
              <Folder v-if="data.isDirectory" />
              <Document v-else />
            </ElIcon>
            <span class="flex-1 truncate">{{ data.name }}</span>
          </div>
        </template>
      </ElTree>
    </div>

    <!-- Current Path -->
    <div class="border-t border-border bg-surface p-2 text-xs text-text-secondary">
      <div
        class="truncate"
        :title="fileStore.currentDirectory"
      >
        当前目录: {{ fileStore.currentDirectory }}
      </div>
    </div>
  </div>
</template>

<style scoped>
:deep(.el-tree) {
  background: transparent;
}

:deep(.el-tree-node__content) {
  height: 32px;
}

:deep(.el-tree-node__content:hover) {
  background-color: var(--color-surface);
}

:deep(.el-tree-node__expand-icon) {
  font-size: 14px;
  color: var(--color-text-secondary);
}

:deep(.el-tree-node__children) {
  overflow: visible;
}
</style>
