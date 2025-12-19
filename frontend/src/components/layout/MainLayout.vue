<script setup lang="ts">
import { Menu, Setting, Folder, Message, Document } from '@element-plus/icons-vue';
import { ElContainer, ElHeader, ElMain, ElDropdown, ElDropdownMenu, ElDropdownItem } from 'element-plus';
import { ref, onMounted, onUnmounted } from 'vue';
import { useRouter } from 'vue-router';
import { showError, showSuccess } from '@/utils/toast';

import { useAppStore, useFileStore } from '@/stores';
import MainSidebar from '@/components/layout/MainSidebar.vue';
import EditorArea from '@/components/layout/EditorArea.vue';
import BottomTabs from '@/components/layout/BottomTabs.vue';
import type { Workspace } from '@/utils/types';

const appStore = useAppStore();
const fileStore = useFileStore();
const router = useRouter();

// Panel visibility & layout
const showFileExplorer = ref(true);
const showBottomPanel = ref(true);
const sidebarWidth = ref(256);
// 固定底部面板高度（像素），避免位置随窗口变化上下浮动
const bottomPanelHeight = 300;

// Bottom panel tabs
const bottomTabs = [
  { key: 'chat', label: '聊天', icon: Message },
  { key: 'output', label: '输出', icon: Document },
  { key: 'terminal', label: '终端', icon: Message },
];

const activeBottomTab = ref('chat');

// Recent directories
const recentDirectories = ref<Workspace[]>([]);

// Toggle panels
function toggleFileExplorer() {
  showFileExplorer.value = !showFileExplorer.value;
}

// Sidebar resize handlers
// function onSidebarResizeStart(event: MouseEvent) {
//   isResizingSidebar.value = true;
//   sidebarStartX = event.clientX;
//   sidebarStartWidth = sidebarWidth.value;

//   document.addEventListener('mousemove', onSidebarResizing);
//   document.addEventListener('mouseup', onSidebarResizeEnd);
// }

// function onSidebarResizing(event: MouseEvent) {
//   if (!isResizingSidebar.value) return;

//   const delta = event.clientX - sidebarStartX;
//   const minWidth = 180;
//   const maxWidth = 480;
//   let nextWidth = sidebarStartWidth + delta;

//   if (nextWidth < minWidth) nextWidth = minWidth;
//   if (nextWidth > maxWidth) nextWidth = maxWidth;

//   sidebarWidth.value = nextWidth;
// }

// function onSidebarResizeEnd() {
//   if (!isResizingSidebar.value) return;

//   isResizingSidebar.value = false;
//   document.removeEventListener('mousemove', onSidebarResizing);
//   document.removeEventListener('mouseup', onSidebarResizeEnd);
// }

// Open settings page
function openSettings() {
  router.push('/settings');
}

// Load recent directories from backend
async function loadRecentDirectories() {
  try {
    recentDirectories.value = appStore.workspaces;
  } catch (error) {
    console.error('加载最近目录失败', error);
    recentDirectories.value = [];
  }
}

// Open a recent directory from header dropdown
async function openRecentDirectoryFromHeader(dir: Workspace) {
  try {
    // 先设置根目录
    fileStore.setRootDirectory(dir.path);
    // 加载目录内容，这会触发 FileExplorer 的 watch 更新
    await fileStore.loadDirectory(dir.path);
    // 切换工作区
    await appStore.switchWorkspace(dir.id);

    showSuccess(`已切换到工作区: ${dir.name}`);
  } catch (error) {
    showError((error as Error).message || '打开目录失败', '打开目录失败');
    console.error('打开目录失败', error);
  }
}

function handleSelectRecentDirectory(command: Workspace) {
  if (command && command.path) {
    openRecentDirectoryFromHeader(command);
  }
}

onMounted(() => {
  loadRecentDirectories();
  
  // Listen for switch-to-terminal event from FileExplorer
  window.addEventListener('switch-to-terminal', handleSwitchToTerminal);
});

onUnmounted(() => {
  // Clean up event listener
  window.removeEventListener('switch-to-terminal', handleSwitchToTerminal);
});

// Handle switching to terminal tab from FileExplorer
function handleSwitchToTerminal(event: Event) {
  const customEvent = event as CustomEvent;
  activeBottomTab.value = 'terminal';
  showBottomPanel.value = true;
  
  // Emit event to TerminalPanel to open terminal in the specified path
  if (customEvent.detail?.path) {
    window.dispatchEvent(new CustomEvent('open-terminal-in-path', {
      detail: { path: customEvent.detail.path }
    }));
  }
}
</script>

<template>
  <ElContainer class="h-full w-full">
    <!-- Header -->
    <ElHeader class="flex items-center justify-between border-b border-border bg-surface px-4">
      <div class="flex items-center space-x-4">
        <div class="flex items-center space-x-2">
          <img src="/vite.svg" class="h-8 w-8" alt="Logo" />
          <span class="text-lg font-semibold">Code AI Assistant</span>
        </div>

        <div class="flex items-center space-x-2">
          <el-button :icon="Menu" text @click="toggleFileExplorer">
            {{ showFileExplorer ? '隐藏导航' : '显示导航' }}
          </el-button>
        </div>
      </div>

      <div class="flex items-center space-x-4">
        <ElDropdown
          v-if="recentDirectories.length > 0"
          trigger="click"
          @command="handleSelectRecentDirectory"
        >
          <span class="recent-dropdown-trigger">
            <el-icon class="mr-1">
              <Folder />
            </el-icon>
            <span class="recent-dropdown-label"> 最近目录 </span>
          </span>
          <template #dropdown>
            <ElDropdownMenu class="recent-dropdown-menu">
              <ElDropdownItem v-for="dir in recentDirectories" :key="dir.path" :command="dir">
                <div class="recent-dir-item">
                  <div class="recent-dir-path">
                    {{ dir.path }}
                  </div>
                  <div class="recent-dir-time">
                    {{ new Date(dir.createdAt).toLocaleString('zh-CN') }}
                  </div>
                </div>
              </ElDropdownItem>
            </ElDropdownMenu>
          </template>
        </ElDropdown>

        <el-button-group>
          <el-button type="primary" disabled> 编辑器 </el-button>
          <el-button @click="openSettings">
            <el-icon><Setting /></el-icon>
            设置
          </el-button>
        </el-button-group>
      </div>
    </ElHeader>

    <!-- Main Content -->
    <ElContainer class="flex-1">
      <MainSidebar :visible="showFileExplorer" :width="sidebarWidth" />

      <!-- Main Content Area -->
      <ElMain class="flex-1 overflow-hidden min-w-0">
        <!-- Editor View -->
        <div class="h-full flex flex-col" :style="{ paddingBottom: bottomPanelHeight + 32 + 'px' }">
          <!-- Editor Area -->
          <EditorArea />

          <BottomTabs
            :tabs="bottomTabs"
            :active-tab="activeBottomTab"
            :visible="showBottomPanel"
            :height="bottomPanelHeight"
            :show-file-explorer="showFileExplorer"
            :sidebar-width="sidebarWidth"
            @update:activeTab="activeBottomTab = $event"
            @update:visible="showBottomPanel = $event"
          />
        </div>
      </ElMain>
    </ElContainer>
  </ElContainer>
</template>

<style scoped>
:deep(.el-header) {
  padding: 0;
  height: 48px;
}

:deep(.el-aside) {
  width: 256px;
}

:deep(.el-main) {
  padding: 0;
}

.recent-dropdown-trigger {
  display: inline-flex;
  align-items: center;
  padding: 4px 10px;
  border-radius: 999px;
  border: 1px solid var(--color-border);
  cursor: pointer;
  font-size: 13px;
  color: var(--color-text-secondary);
  transition: all 0.15s ease-in-out;
}

.recent-dropdown-trigger:hover {
  background-color: rgba(0, 0, 0, 0.04);
  color: var(--color-text);
}

.recent-dropdown-label {
  max-width: 160px;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.recent-dir-item {
  display: flex;
  flex-direction: column;
  max-width: 320px;
}

.recent-dir-path {
  font-size: 13px;
  font-weight: 500;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.recent-dir-time {
  margin-top: 2px;
  font-size: 12px;
  color: var(--color-text-secondary);
}

.sidebar-resizer {
  width: 4px;
  cursor: col-resize;
  background-color: transparent;
}

.editor-root {
  position: relative;
}
</style>
