<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import {
  WarningFilled,
  CircleCloseFilled,
  InfoFilled,
  SuccessFilled,
  Close,
} from '@element-plus/icons-vue';
import { ElIcon } from 'element-plus';

export interface ErrorToastProps {
  type?: 'error' | 'warning' | 'info' | 'success';
  title?: string;
  message: string;
  duration?: number; // 显示时长（毫秒），0 表示不自动关闭
  closable?: boolean; // 是否可手动关闭
  showIcon?: boolean; // 是否显示图标
  onClose?: () => boolean; // 关闭回调
}

const props = withDefaults(defineProps<ErrorToastProps>(), {
  type: 'error',
  title: '',
  duration: 3000,
  closable: true,
  showIcon: true,
});

const emit = defineEmits<{
  close: [];
}>();

const visible = ref(false);
let timer: ReturnType<typeof setTimeout> | null = null;

// 根据类型获取对应的图标
const icon = computed(() => {
  switch (props.type) {
    case 'error':
      return CircleCloseFilled;
    case 'warning':
      return WarningFilled;
    case 'info':
      return InfoFilled;
    case 'success':
      return SuccessFilled;
    default:
      return CircleCloseFilled;
  }
});

// 根据类型获取对应的样式类
const typeClass = computed(() => {
  return {
    'border-red-500 bg-red-50 dark:bg-red-950': props.type === 'error',
    'border-yellow-500 bg-yellow-50 dark:bg-yellow-950': props.type === 'warning',
    'border-blue-500 bg-blue-50 dark:bg-blue-950': props.type === 'info',
    'border-green-500 bg-green-50 dark:bg-green-950': props.type === 'success',
  };
});

// 图标颜色类
const iconClass = computed(() => {
  return {
    'text-red-500': props.type === 'error',
    'text-yellow-500': props.type === 'warning',
    'text-blue-500': props.type === 'info',
    'text-green-500': props.type === 'success',
  };
});

// 默认标题
const displayTitle = computed(() => {
  if (props.title) return props.title;
  switch (props.type) {
    case 'error':
      return '错误';
    case 'warning':
      return '警告';
    case 'info':
      return '提示';
    case 'success':
      return '成功';
    default:
      return '提示';
  }
});

// 关闭提示
function close() {
  visible.value = false;
  if (timer) {
    clearTimeout(timer);
    timer = null;
  }
  emit('close');
  if (props.onClose) {
    props.onClose();
  }
}

// 启动自动关闭计时器
function startTimer() {
  if (props.duration > 0) {
    timer = setTimeout(() => {
      close();
    }, props.duration);
  }
}

// 组件挂载时显示并启动计时器
onMounted(() => {
  visible.value = true;
  startTimer();
});
</script>

<template>
  <Transition name="slide-fade">
    <div
      v-if="visible"
      class="error-toast border-l-4 rounded-lg shadow-lg p-4 mb-3 flex items-start space-x-3"
      :class="typeClass"
      role="alert"
    >
      <!-- Icon -->
      <ElIcon v-if="showIcon" :size="20" class="flex-shrink-0 mt-0.5" :class="iconClass">
        <component :is="icon" />
      </ElIcon>

      <!-- Content -->
      <div class="flex-1 min-w-0">
        <h4 class="font-semibold text-sm mb-1 text-gray-900 dark:text-gray-100">
          {{ displayTitle }}
        </h4>
        <p class="text-sm text-gray-700 dark:text-gray-300 break-words">
          {{ message }}
        </p>
      </div>

      <!-- Close Button -->
      <button
        v-if="closable"
        class="flex-shrink-0 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200 transition-colors"
        @click="close"
      >
        <ElIcon :size="16">
          <Close />
        </ElIcon>
      </button>
    </div>
  </Transition>
</template>

<style scoped>
.error-toast {
  max-width: 400px;
  min-width: 300px;
  backdrop-filter: blur(8px);
}

/* 滑动淡入淡出动画 */
.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.2s ease-in;
}

.slide-fade-enter-from {
  transform: translateX(20px);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateY(-10px);
  opacity: 0;
}
</style>
