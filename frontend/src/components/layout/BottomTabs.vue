<script setup lang="ts">
import { ElButton, ElButtonGroup } from 'element-plus';
// import { computed } from 'vue';

import ChatPanel from '@/components/chat/ChatPanel.vue';
import OutputPanel from '@/components/output/OutputPanel.vue';
import TerminalPanel from '@/components/terminal/TerminalPanel.vue';
import { useAppStore, useFileStore } from '@/stores';

interface BottomTab {
  key: string;
  label: string;
  icon: any;
}

const props = defineProps<{
  tabs: BottomTab[];
  activeTab: string;
  visible: boolean;
  height: number;
}>();

const emit = defineEmits<{
  (e: 'update:activeTab', value: string): void;
  (e: 'update:visible', value: boolean): void;
}>();

const appStore = useAppStore();
const fileStore = useFileStore();

// const currentFile = computed(() => fileStore.currentFile);

function onToggleVisible() {
  emit('update:visible', !props.visible);
}

function onSelectTab(key: string) {
  emit('update:activeTab', key);
}
</script>

<template>
  <div class="flex flex-col flex-shrink-0">
    <!-- Bottom Panel Toggle Bar -->
    <div class="border-t border-border bg-surface px-4 py-1 flex-shrink-0">
      <div class="flex items-center justify-between">
        <ElButtonGroup>
          <ElButton
            v-for="tab in props.tabs"
            :key="tab.key"
            :type="props.activeTab === tab.key ? 'primary' : 'default'"
            :icon="tab.icon"
            size="small"
            @click="onSelectTab(tab.key)"
          >
            {{ tab.label }}
          </ElButton>
        </ElButtonGroup>

        <ElButton
          :icon="props.visible ? 'ArrowDown' : 'ArrowUp'"
          size="small"
          text
          @click="onToggleVisible"
        >
          {{ props.visible ? '隐藏面板' : '显示面板' }}
        </ElButton>
      </div>
    </div>

    <!-- Bottom Panel Content -->
    <div
      v-if="props.visible"
      class="border-t border-border bg-background overflow-hidden flex-shrink-0"
      :style="{ height: props.height + 'px' }"
    >
      <ChatPanel v-if="props.activeTab === 'chat'" />
      <OutputPanel v-else-if="props.activeTab === 'output'" />
      <TerminalPanel v-else-if="props.activeTab === 'terminal'" />
    </div>

    <!-- Footer Status Bar -->
    <div
      class="border-t border-border bg-surface px-4 py-2 text-sm text-text-secondary flex-shrink-0"
    >
      <div class="flex items-center justify-between">
        <div class="flex items-center space-x-4">
          <span>工作区: {{ appStore.currentWorkspace.name }}</span>
          <span>|</span>
          <span>文件: {{ fileStore.getCurrentFile?.path || '未选择文件' }}</span>
        </div>

        <div class="flex items-center space-x-4">
          <span>AI模型:{{ appStore.currentAiModel }}</span>
          <span>|</span>
          <span>状态: {{ appStore.isConnected ? '已连接' : '未连接' }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
/* 所有样式都移除，使用 flex 布局代替 fixed 定位 */
</style>
