<script setup lang="ts">
import { ref } from 'vue';
import ErrorToast, { type ErrorToastProps } from './ErrorToast.vue';

export interface ToastItem extends ErrorToastProps {
  id: string;
}

const toasts = ref<ToastItem[]>([]);

// 添加提示
function addToast(options: Omit<ToastItem, 'id'>) {
  const id = `toast-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
  const toast: ToastItem = {
    id,
    ...options,
  };

  toasts.value.push(toast);

  // 如果有 duration，在指定时间后自动移除（加一点缓冲时间）
  if (options.duration && options.duration > 0) {
    setTimeout(() => {
      removeToast(id);
    }, options.duration + 500);
  }
}

// 移除提示
function removeToast(id: string) {
  const index = toasts.value.findIndex((t) => t.id === id);
  if (index > -1) {
    toasts.value.splice(index, 1);
  }
}

// 清空所有提示
function clearAll() {
  toasts.value = [];
}

// 暴露方法给父组件
defineExpose({
  addToast,
  removeToast,
  clearAll,
});
</script>

<template>
  <div class="error-container fixed top-4 right-4 z-50 pointer-events-none">
    <div class="pointer-events-auto space-y-3">
      <ErrorToast
        v-for="toast in toasts"
        :key="toast.id"
        :type="toast.type"
        :title="toast.title"
        :message="toast.message"
        :duration="toast.duration"
        :closable="toast.closable"
        :show-icon="toast.showIcon"
        @close="removeToast(toast.id)"
      />
    </div>
  </div>
</template>

<style scoped>
.error-container {
  max-height: calc(100vh - 2rem);
  overflow-y: auto;
}

/* 隐藏滚动条但保持功能 */
.error-container::-webkit-scrollbar {
  display: none;
}

.error-container {
  -ms-overflow-style: none;
  scrollbar-width: none;
}
</style>
