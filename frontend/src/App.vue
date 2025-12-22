<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { useAppStore, useThemeStore, useChatStore } from '@/stores';
import { ErrorContainer } from '@/components/error';
import { setToastContainer, showSuccess, showError } from '@/utils/toast';
import { initLightweightModeMonitor } from '@/services/tauri/lightweightMode';
import { initTauriEventListeners } from '@/services/tauri/eventListeners';

const appStore = useAppStore();
const themeStore = useThemeStore();
const chatStore = useChatStore();
const isLoading = ref(true);
const toastContainerRef = ref<InstanceType<typeof ErrorContainer> | null>(null);
let disposeLightweightMode: null | (() => void) = null;
let disposeTauriEvents: null | (() => void) = null;

onMounted(async () => {
  // 初始化 toast 容器
  if (toastContainerRef.value) {
    setToastContainer(toastContainerRef.value);
  }
  try {
    // Initialize application
    await appStore.initialize();
    // Theme
    themeStore.initialize(appStore.settings.theme, appStore.settings.colorScheme);

    // Lightweight mode (minimized/hidden throttling)
    disposeLightweightMode = await initLightweightModeMonitor(appStore);

    disposeTauriEvents = await initTauriEventListeners({
      appStore,
      chatStore,
    });

    showSuccess('Code AI Assistant 已准备就绪', '应用启动成功');
  } catch (error) {
    console.error('Failed to initialize app:', error);
    showError(error instanceof Error ? error.message : '未知错误', '应用启动失败', 5000);
  } finally {
    // Always show the layout even if initialization fails
    isLoading.value = false;
  }
});

onUnmounted(() => {
  disposeLightweightMode?.();
  disposeLightweightMode = null;
  disposeTauriEvents?.();
  disposeTauriEvents = null;
});

</script>

<template>
  <div id="app">
    <div v-if="isLoading" class="flex-col-center h-full w-full">
      <div
        class="animate-spin rounded-full h-16 w-16 border-t-2 border-b-2 border-primary-500"
      ></div>
      <p class="mt-4 text-text-secondary">正在初始化应用...</p>
    </div>

    <router-view v-else />

    <!-- 全局错误提示容器 -->
    <ErrorContainer ref="toastContainerRef" />
  </div>
</template>

<style scoped>
#app {
  height: 100vh;
  width: 100vw;
  overflow: hidden;
}
</style>
