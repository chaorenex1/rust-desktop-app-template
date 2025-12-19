<script setup lang="ts">
import { Folder, Document, Plus, Refresh, Search, More, Edit, Delete, CopyDocument, Monitor } from '@element-plus/icons-vue';
import { ElTree, ElInput, ElButton, ElIcon, ElMessageBox } from 'element-plus';
import { ref, onMounted, watch } from 'vue';
import { useFileStore } from '@/stores/filesStore';
import { useAppStore } from '@/stores/appStore';
import { showError, showWarning, showSuccess } from '@/utils/toast';

import { listFiles, readFile, writeFile, createDirectory, deleteFile, renameFile, deleteDirectory } from '@/services/tauri/commands';
import type { FileItem } from '@/utils/types';
import { getIcon } from '@/utils/fileIcons';

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
  icon?: FileIconConfig;
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
// watch(
//   () => fileStore.getDirectoryChildren(fileStore.getRootDirectory()),
//   (files) => {
//     treeData.value = files.map((file) => ({
//       name: file.name,
//       path: file.path,
//       isDirectory: file.isDirectory,
//       size: file.size,
//       modified: file.modified,
//       isLeaf: !file.isDirectory,
//     }));
//   },
//   { deep: true }
// );

// 监听根目录变化，重新初始化
watch(
  () => fileStore.getRootDirectory(),
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

  treeData.value = fileStore.files.map((file) => ({
    name: file.name,
    path: file.path,
    isDirectory: file.isDirectory,
    size: file.size,
    modified: file.modified,
    isLeaf: !file.isDirectory,
    icon: getIcon(file.name, file.isDirectory),
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
    level: node.level,
  });

  try {
    // 使用 filesStore 的 loadSubDirectory，支持缓存
    const fileList = await fileStore.loadSubDirectory(data.path);
    console.debug(`Loaded ${fileList.length} items from ${data.path}`);

    const children: FileNode[] = fileList.map((file) => ({
      name: file.name,
      path: file.path,
      isDirectory: file.isDirectory,
      size: file.size,
      modified: file.modified,
      isLeaf: !file.isDirectory,
      icon: getIcon(file.name, file.isDirectory),
    }));

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
      } else {
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

// Handle context menu commands
async function handleContextCommand(command: string, data: FileNode) {
  console.log('Context command:', command, data);
  
  switch (command) {
    case 'rename':
      await handleRename(data);
      break;
    case 'delete':
      await handleDelete(data);
      break;
    case 'copy_path':
      await handleCopyPath(data);
      break;
    case 'open_terminal':
      await handleOpenTerminal(data);
      break;
  }
}

// Rename file or directory
async function handleRename(data: FileNode) {
  try {
    const result = await ElMessageBox.prompt(
      `请输入新的${data.isDirectory ? '文件夹' : '文件'}名称`,
      '重命名',
      {
        confirmButtonText: '确定',
        cancelButtonText: '取消',
        inputValue: data.name,
        inputPattern: /^[^\\/:*?"<>|]+$/,
        inputErrorMessage: '名称不能包含特殊字符: \\ / : * ? " < > |',
      }
    );

    if (result.value && result.value !== data.name) {
      const newName = result.value.trim();
      const parentPath = data.path.substring(0, data.path.lastIndexOf('/'));
      const newPath = `${parentPath}/${newName}`;
      
      await fileStore.renameFile(data.path, newName);
      showSuccess('重命名成功');
      await initialize(true);
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      showError(
        error instanceof Error ? error.message : '重命名失败',
        '重命名失败'
      );
    }
  }
}

// Delete file or directory
async function handleDelete(data: FileNode) {
  try {
    const confirmResult = await ElMessageBox.confirm(
      `确定要删除 "${data.name}" 吗？此操作不可恢复。`,
      '确认删除',
      {
        confirmButtonText: '删除',
        cancelButtonText: '取消',
        type: 'warning',
        buttonSize: 'default',
      }
    );

    if (confirmResult) {
      if (data.isDirectory) {
        await deleteDirectory(data.path);
      } else {
        await fileStore.deleteFile(data.path);
      }
      
      showSuccess('删除成功');
      await initialize(true);
    }
  } catch (error: any) {
    if (error !== 'cancel') {
      showError(
        error instanceof Error ? error.message : '删除失败',
        '删除失败'
      );
    }
  }
}

// Copy path to clipboard
async function handleCopyPath(data: FileNode) {
  try {
    if (navigator.clipboard && navigator.clipboard.writeText) {
      await navigator.clipboard.writeText(data.path);
      showSuccess('路径已复制到剪贴板');
    } else {
      // Fallback for older browsers
      const textArea = document.createElement('textarea');
      textArea.value = data.path;
      textArea.style.position = 'fixed';
      textArea.style.left = '-9999px';
      document.body.appendChild(textArea);
      textArea.select();
      document.execCommand('copy');
      document.body.removeChild(textArea);
      showSuccess('路径已复制到剪贴板');
    }
  } catch (error) {
    showError('复制路径失败', '复制失败');
  }
}

// Open terminal in directory
async function handleOpenTerminal(data: FileNode) {
  try {
    if (!data.isDirectory) {
      showWarning('只能在文件夹中打开终端');
      return;
    }
    
    // Emit custom event to notify MainLayout to switch to terminal tab
    window.dispatchEvent(new CustomEvent('switch-to-terminal', {
      detail: { path: data.path }
    }));
    
    showSuccess(`已在 "${data.name}" 打开终端`);
  } catch (error) {
    showError('打开终端失败', '操作失败');
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Toolbar -->
    <div class="border-b border-border bg-surface p-2">
      <div class="flex items-center justify-between mb-2">
        <div class="flex items-center space-x-1">
          <ElButton :icon="Plus" size="small" text @click="createNew(false)"> 文件 </ElButton>
          <ElButton :icon="Folder" size="small" text @click="createNew(true)"> 文件夹 </ElButton>
        </div>
        <ElButton :icon="Refresh" size="small" text @click="refreshDirectory" />
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
      <div v-if="fileStore.isLoading" class="flex flex-col items-center justify-center h-full">
        <div class="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-primary-500" />
        <p class="mt-2 text-sm text-text-secondary">加载中...</p>
      </div>

      <div v-else-if="fileStore.error && !treeData.length" class="text-center p-4 text-error">
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
          <div class="tree-node-content flex items-center px-1 py-1 w-full">
            <span
              class="mr-2 text-base leading-none flex-shrink-0"
              :style="{ color: data.icon.color }"
            >
              {{ data.icon.icon }}
            </span>
            <span class="flex-1 truncate">{{ data.name }}</span>
            <div class="context-menu-trigger">
              <ElDropdown
                trigger="click"
                @command="(command) => handleContextCommand(command, data)"
              >
                <span class="el-dropdown-link context-menu-btn">
                  <ElIcon :size="16"><More /></ElIcon>
                </span>
                <template #dropdown>
                  <ElDropdownMenu>
                    <ElDropdownItem command="rename">
                      <ElIcon class="mr-2"><Edit /></ElIcon>
                      重命名
                    </ElDropdownItem>
                    <ElDropdownItem command="copy_path">
                      <ElIcon class="mr-2"><CopyDocument /></ElIcon>
                      复制路径
                    </ElDropdownItem>
                    <ElDropdownItem command="open_terminal" v-if="data.isDirectory">
                      <ElIcon class="mr-2"><Monitor /></ElIcon>
                      在终端打开
                    </ElDropdownItem>
                    <ElDropdownItem command="delete" divided>
                      <ElIcon class="mr-2"><Delete /></ElIcon>
                      <span class="text-red-500">删除</span>
                    </ElDropdownItem>
                  </ElDropdownMenu>
                </template>
              </ElDropdown>
            </div>
          </div>
        </template>
      </ElTree>
    </div>

    <!-- Current Path -->
    <div class="border-t border-border bg-surface p-2 text-xs text-text-secondary">
      <div class="truncate" :title="fileStore.currentDirectory">
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
  position: relative;
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

/* Context menu styling */
.tree-node-content {
  position: relative;
}

.context-menu-trigger {
  opacity: 0;
  transition: opacity 0.2s ease;
  margin-left: 8px;
}

.tree-node-content:hover .context-menu-trigger {
  opacity: 1;
}

.context-menu-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  padding: 4px;
  border-radius: 4px;
  cursor: pointer;
  color: var(--color-text-secondary);
  transition: all 0.2s ease;
}

.context-menu-btn:hover {
  background-color: var(--el-fill-color-light);
  color: var(--color-text);
}

/* Dropdown menu styling */
:deep(.el-dropdown-menu__item) {
  display: flex;
  align-items: center;
  padding: 8px 16px;
}

:deep(.el-dropdown-menu__item .el-icon) {
  font-size: 14px;
}
</style>
