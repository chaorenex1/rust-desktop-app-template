<script setup lang="ts">
import { ElAside } from 'element-plus';
import FileExplorer from '@/components/file-explorer/FileExplorer.vue';

const props = defineProps<{
  visible: boolean;
  width: number;
}>();

const emit = defineEmits<{
  resizeStart: [event: MouseEvent];
}>();

function handleResizeStart(event: MouseEvent) {
  emit('resizeStart', event);
}
</script>

<template>
  <template v-if="props.visible">
    <ElAside
      class="border-r border-border bg-surface overflow-auto"
      :style="{ width: props.width + 'px' }"
    >
      <FileExplorer />
    </ElAside>

    <div 
      class="sidebar-resizer" 
      @mousedown="handleResizeStart"
      @dblclick="() => emit('resize-reset')"
    />
  </template>
</template>

<style scoped>
.sidebar-resizer {
  width: 6px;
  cursor: col-resize;
  background-color: transparent;
  position: relative;
  transition: background-color 0.15s ease;
  user-select: none;
}

.sidebar-resizer:hover {
  background-color: var(--el-color-primary-light-7, rgba(64, 158, 255, 0.2));
}

.sidebar-resizer:active {
  background-color: var(--el-color-primary-light-5, rgba(64, 158, 255, 0.4));
}

/* 添加视觉提示线 */
.sidebar-resizer::before {
  content: '';
  position: absolute;
  left: 50%;
  top: 50%;
  transform: translate(-50%, -50%);
  width: 2px;
  height: 40px;
  background-color: var(--color-border);
  border-radius: 2px;
  opacity: 0;
  transition: opacity 0.15s ease;
  pointer-events: none;
}

.sidebar-resizer:hover::before {
  opacity: 0.6;
}
</style>
