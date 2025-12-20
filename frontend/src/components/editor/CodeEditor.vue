<script setup lang="ts">
import { Document, FolderOpened } from '@element-plus/icons-vue';
import { ElTabs, ElTabPane, ElButton, ElIcon, ElTooltip } from 'element-plus';
import monaco from '@/utils/monaco';
import { ref, onBeforeUnmount, onBeforeUpdate, watch, nextTick, computed, toRaw } from 'vue';

import { useFileStore, useAppStore } from '@/stores';

const emit = defineEmits<{
  'load-more': [];
}>();

const fileStore = useFileStore();
const appStore = useAppStore();
const editorContainer = ref<HTMLElement>();
const editor = ref<monaco.editor.IStandaloneCodeEditor>();
const isLoading = ref(false);
const contentChangeDisposable = ref<monaco.IDisposable>(); // 存储事件监听器

// 处理路径，兼容 Windows 和 POSIX
function getFileNameFromPath(path: string): string {
  const parts = path.split(/[/\\]/);
  return parts[parts.length - 1] || path;
}

const duplicateNames = computed(() => {
  const counts = new Map<string, number>();
  for (const file of fileStore.openedFiles) {
    const name = getFileNameFromPath(file.path);
    counts.set(name, (counts.get(name) || 0) + 1);
  }
  return new Set<string>(
    Array.from(counts.entries())
      .filter(([, count]) => count > 1)
      .map(([name]) => name)
  );
});

// // onMounted
// onMounted(() => {
//   // 等待 DOM 更新，确保容器已渲染
//   editor.value = monaco.editor.create(editorContainer.value, {
//             value: toRaw(newFile.content), // 使用 toRaw 获取原始值
//             language: newFile.language || 'plaintext',
//             theme: 'vs',
//             fontSize: 14,
//             lineNumbers: 'on',
//             wordWrap: 'on',
//             minimap: { enabled: true },
//             scrollBeyondLastLine: false,
//             automaticLayout: true,
//             formatOnPaste: true,
//             formatOnType: true,
//           });
// });

// Watch for active file index changes to switch files

watch(
  () => fileStore.activeFileIndex,
  async (newIndex, oldIndex) => {
    if (newIndex === oldIndex) return;

    const newFile = fileStore.activeFile;

    if (newFile) {
      await initialize();
      if (editor.value) {
        // 切换文件：更新编辑器内容
        const model = getRawEditor()?.getModel();
        if (model) {
          // 使用 toRaw 获取原始值，防止 Vue Proxy 导致 Monaco 卡死
          getRawEditor()?.setValue(toRaw(newFile.content));
          monaco.editor.setModelLanguage(model, toRaw(newFile.language) || 'plaintext');
          console.debug('File content updated in editor');
        }
      }
    } else {
      // 没有活动文件时，销毁编辑器
      if (editor.value) {
        console.debug('No active file, destroying editor...');
        getRawEditor()?.dispose();
        editor.value = undefined; // 清空引用，确保下次能重新创建
        console.debug('Editor destroyed');
      }
    }
  },
  { immediate: true }
);

// 组件卸载时销毁实例
onBeforeUnmount(() => {
  if (contentChangeDisposable.value) {
    contentChangeDisposable.value.dispose();
  }
  if (editor.value) {
    getRawEditor()?.dispose();
  }
});

onBeforeUpdate(async () => {
  console.debug('CodeEditor is about to update');
  // await reinitialize();
});

async function initialize() {
  // 等待 DOM 更新，确保容器已渲染
  await nextTick();
  // 首次创建编辑器
  console.debug('code editor instance:', getRawEditor());
  if (!editor.value && editorContainer.value) {
    console.debug('Initializing CodeEditor...');
    try {
      editor.value = monaco.editor.create(editorContainer.value, {
        value: '',
        language: 'plaintext',
        theme: 'vs',
        fontSize: appStore.settings.editor.fontSize,
        lineNumbers: appStore.settings.editor.lineNumbers,
        wordWrap: appStore.settings.editor.wordWrap,
        minimap: appStore.settings.editor.minimap,
        scrollBeyondLastLine: false,
        automaticLayout: true,
        formatOnPaste: true,
        formatOnType: true,
      });

      // 获取编辑器的配置选项
      // const options = editor.value.getOptions();

      // // 获取行高
      // const lineHeight = options.get(monaco.editor.EditorOption.lineHeight);

      // // 节流函数
      // function throttle(func: Function, wait: number) {
      //   let lastTime = 0;
      //   return function(this: any, ...args: any[]) {
      //     const now = Date.now();
      //     if (now - lastTime >= wait) {
      //       lastTime = now;
      //       func.apply(this, args);
      //     }
      //   };
      // }

      // // 监听滚动事件
      // editor.value.onDidScrollChange(throttle((e: any) => {
      //   const layoutInfo = editor.value?.getLayoutInfo();
      //   const scrollTop = e.scrollTop; // 当前滚动位置
      //   const visibleHeight = layoutInfo?.height || 0; // 可见区域的高度

      //   const model = editor.value?.getModel();
      //   const lineCount = model?.getLineCount() || 0; // 总行数

      //   // 判断是否滑到底部
      //   if (scrollTop >= lineCount * lineHeight - visibleHeight && lineCount > 0 && visibleHeight !== undefined) {
      //     console.log('滚动条滑到底部了！');
      //     emit('load-more'); // 发出 load-more 事件
      //   }
      // }, 1000)); // 1000ms 的节流间隔
    } catch (error) {
      console.error('Failed to create Monaco Editor:', error);
    }
  }
}

async function reinitialize() {
  if (contentChangeDisposable.value) {
    contentChangeDisposable.value.dispose();
  }
  if (editor.value) {
    getRawEditor()?.dispose();
    await initialize();
    console.debug('Reinitializing CodeEditor...');
  }
}

function getRawEditor(): monaco.editor.IStandaloneCodeEditor | undefined {
  return toRaw(editor.value)
}

// Save current file
async function saveCurrentFile() {
  if (!fileStore.activeFile) {
    return;
  }

  try {
    isLoading.value = true;
    const content = editor.value?.getValue() || '';
    await fileStore.saveFile(content);
  } catch (error) {
    console.error('Failed to save file:', error);
  } finally {
    isLoading.value = false;
  }
}

// Save all files
async function saveAllFiles() {
  try {
    isLoading.value = true;
    await fileStore.saveAllFiles();
  } catch (error) {
    console.error('Failed to save all files:', error);
  } finally {
    isLoading.value = false;
  }
}

// Close file
function closeFile(index: number) {
  const file = fileStore.openedFiles[index];
  if (!file) {
    return;
  }

  if (file.modified) {
    if (!confirm('文件有未保存的更改，确定要关闭吗？')) {
      return;
    }
  }
  fileStore.closeFile(file.path);
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- File Tabs -->
    <div class="border-b border-border bg-surface">
      <ElTabs
        v-model="fileStore.activeFileIndex"
        type="card"
        closable
        @tab-remove="(name: any) => closeFile(name as number)"
      >
        <ElTabPane v-for="(file, index) in fileStore.openedFiles" :key="file.path" :name="index">
          <template #label>
            <div class="flex items-center">
              <span class="mr-2 max-w-[180px] truncate" :title="file.path">
                {{ file.name }}
              </span>
              <span v-if="file.modified" class="text-warning">*</span>
            </div>
          </template>
        </ElTabPane>
      </ElTabs>

      <!-- Editor Toolbar -->
      <div class="flex items-center justify-between px-4 py-2 border-t border-border">
        <div class="flex items-center space-x-2">
          <span class="text-sm text-text-secondary">
            {{ fileStore.activeFile?.language || 'plaintext' }}
          </span>
        </div>

        <div class="flex items-center space-x-2">
          <ElTooltip content="保存当前文件 (Ctrl+S)" placement="bottom">
            <ElButton :icon="Document" :loading="isLoading" size="small" @click="saveCurrentFile">
              保存
            </ElButton>
          </ElTooltip>

          <ElTooltip content="保存所有文件 (Ctrl+Shift+S)" placement="bottom">
            <ElButton :icon="FolderOpened" :loading="isLoading" size="small" @click="saveAllFiles">
              全部保存
            </ElButton>
          </ElTooltip>
        </div>
      </div>
    </div>

    <!-- Editor Area -->
    <div class="flex-1 overflow-hidden min-h-0">
      <div
        v-if="!fileStore.activeFile"
        class="flex flex-col items-center justify-center h-full text-text-secondary"
      >
        <ElIcon :size="48" class="mb-4">
          <Document />
        </ElIcon>
        <p>打开一个文件开始编辑</p>
      </div>

      <div v-else ref="editorContainer" class="h-full w-full" />
    </div>

    <!-- Status Bar -->
    <div class="border-t border-border bg-surface px-4 py-1 text-xs text-text-secondary">
      <div class="flex items-center justify-between">
        <div>
          <span v-if="fileStore.activeFile">
            行: {{ editor?.getModel()?.getLineCount() || 0 }} | 列:
            {{ editor?.getPosition()?.column || 1 }}
          </span>
        </div>

        <div>
          <span v-if="fileStore.activeFile?.modified" class="text-warning"> 有未保存的更改 </span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
:deep(.el-tabs__header) {
  margin: 0;
}

:deep(.el-tabs__nav-wrap) {
  padding: 0 16px;
}

:deep(.el-tabs__item) {
  padding: 0 12px;
  height: 32px;
  line-height: 32px;
}
</style>
