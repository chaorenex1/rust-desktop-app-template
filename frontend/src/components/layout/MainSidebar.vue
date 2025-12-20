<script setup lang="ts">
import { ElAside } from 'element-plus';
import FileExplorer from '@/components/file-explorer/FileExplorer.vue';
import { ref } from 'vue';

const props = defineProps<{
  visible: boolean;
  width: number;
}>();

const emit = defineEmits<{
  resizeStart: [event: MouseEvent];
  updateWidth: [width: number];
}>();

// 拉伸功能相关
const sidebarWidth = ref(props.width);
const isResizing = ref(false);
let resizeStartX = 0;
let resizeStartWidth = 0;

function handleResizeStart(event: MouseEvent) {
  isResizing.value = true;
  resizeStartX = event.clientX;
  resizeStartWidth = sidebarWidth.value;

  document.addEventListener('mousemove', handleResizing);
  document.addEventListener('mouseup', handleResizeEnd);

  event.preventDefault();
}

function handleResizing(event: MouseEvent) {
  if (!isResizing.value) return;

  // 向右拉伸：鼠标右移增加宽度，左移减少宽度
  const delta = event.clientX - resizeStartX;
  const minWidth = 180;
  const maxWidth = 600;
  let nextWidth = resizeStartWidth + delta;

  if (nextWidth < minWidth) nextWidth = minWidth;
  if (nextWidth > maxWidth) nextWidth = maxWidth;

  sidebarWidth.value = nextWidth;
  emit('updateWidth', nextWidth);
}

function handleResizeEnd() {
  if (!isResizing.value) return;

  isResizing.value = false;
  document.removeEventListener('mousemove', handleResizing);
  document.removeEventListener('mouseup', handleResizeEnd);
}
</script>

<template>
  <template v-if="props.visible">
    <ElAside
      class="border-r border-border bg-surface overflow-hidden"
      :style="{ width: sidebarWidth + 'px' }"
    >
      <div class="h-full overflow-auto">
        <FileExplorer />
      </div>
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
  /* width: 4px; */
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
  width: 10px;
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
