<script setup lang="ts">
import { Setting, Document, ChatDotRound, Promotion, Folder } from '@element-plus/icons-vue';
import { ElButton, ElCard, ElDialog, ElMessage } from 'element-plus';
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import { open as openDialog } from '@tauri-apps/plugin-dialog';
import type { Workspace } from '@/utils/types';
import { useFileStore, useAppStore } from '@/stores';

const router = useRouter();
const fileStore = useFileStore();
const appStore = useAppStore();

const showDirectoryDialog = ref(false);
const isLoading = ref(false);

const features = [
  {
    icon: ChatDotRound,
    title: 'AI 聊天助手',
    description: '与 AI 对话，获取代码建议和帮助',
  },
  {
    icon: Document,
    title: '智能代码编辑',
    description: '使用 AI 辅助编写和优化代码',
  },
  {
    icon: Setting,
    title: '灵活配置',
    description: '自定义设置，适应不同开发场景',
  },
];

async function startCoding() {
  showDirectoryDialog.value = true;
}

async function selectDirectory() {
  try {
    isLoading.value = true;

    // 使用 Tauri dialog 插件打开目录选择对话框
    const result = await openDialog({
      directory: true,
      multiple: false,
      title: '选择工作目录',
    });

    if (result && typeof result === 'string') {
      // 加载目录
      await fileStore.loadDirectory(result);
      fileStore.setRootDirectory(result);

      // 创建工作区
      await appStore.createWorkspace(result, true);

      // 关闭对话框
      showDirectoryDialog.value = false;

      // 跳转到dashboard
      router.push('/dashboard');
    } else {
      // 用户取消了选择
      showDirectoryDialog.value = false;
    }
  } catch (error) {
    ElMessage.error('选择目录失败: ' + (error as Error).message);
    console.error('选择目录失败', error);
    showDirectoryDialog.value = false;
  } finally {
    isLoading.value = false;
  }
}

async function openRecentDirectory(dir: Workspace) {
  try {
    console.debug('Opening recent directory:', dir);
    await fileStore.loadDirectory(dir.path);
    fileStore.setRootDirectory(dir.path);
    // 创建工作区
    await appStore.switchWorkspace(dir.id);

    router.push('/dashboard');
  } catch (error) {
    ElMessage.error('打开目录失败: ' + (error as Error).message);
    console.error('打开目录失败', error);
  }
}
</script>

<template>
  <div
    class="home-page min-h-screen bg-gradient-to-br from-primary-50 to-primary-100 dark:from-gray-900 dark:to-gray-800"
  >
    <div class="container mx-auto px-4 py-16">
      <!-- Hero Section -->
      <div class="text-center mb-16">
        <h1 class="text-5xl font-bold text-primary-600 dark:text-primary-400 mb-4">
          Code AI Assistant
        </h1>
        <p class="text-xl text-gray-600 dark:text-gray-300 mb-8">
          您的智能代码开发助手，让编程更高效、更简单
        </p>
        <div class="flex justify-center gap-4">
          <ElButton type="primary" size="large" @click="startCoding">
            <el-icon class="mr-2">
              <Promotion />
            </el-icon>
            开始编码
          </ElButton>
        </div>
      </div>

      <!-- Features Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
        <ElCard
          v-for="(feature, index) in features"
          :key="index"
          shadow="hover"
          class="text-center"
        >
          <template #header>
            <div class="flex justify-center">
              <el-icon :size="40" class="text-primary-500">
                <component :is="feature.icon" />
              </el-icon>
            </div>
          </template>
          <h3 class="text-lg font-semibold mb-2">
            {{ feature.title }}
          </h3>
          <p class="text-gray-600 dark:text-gray-400">
            {{ feature.description }}
          </p>
        </ElCard>
      </div>

      <!-- Directory Selection Dialog -->
      <ElDialog
        v-model="showDirectoryDialog"
        title="选择工作目录"
        width="500px"
        :close-on-click-modal="false"
      >
        <div class="py-4">
          <p class="text-gray-600 dark:text-gray-400 mb-4">
            请选择一个目录作为您的工作区，AI助手将帮助您处理该目录中的文件。
          </p>
          <ElButton type="primary" class="w-full" :loading="isLoading" @click="selectDirectory">
            <el-icon class="mr-2">
              <Folder />
            </el-icon>
            选择目录
          </ElButton>
        </div>
      </ElDialog>

      <!-- Recent Directories -->
      <div v-if="appStore.workspaces.length > 0" class="mt-16 max-w-3xl mx-auto">
        <ElCard>
          <template #header>
            <h2 class="text-2xl font-bold text-center">最近打开的目录</h2>
          </template>
          <div class="space-y-2">
            <div
              v-for="(dir, index) in useAppStore().workspaces"
              :key="index"
              class="flex items-center justify-between p-3 hover:bg-gray-50 dark:hover:bg-gray-800 rounded cursor-pointer"
              @click="openRecentDirectory(dir)"
            >
              <div class="flex items-center flex-1">
                <el-icon class="mr-3 text-primary-500">
                  <Folder />
                </el-icon>
                <div class="flex-1 min-w-0">
                  <div class="font-medium truncate">
                    {{ dir.path }}
                  </div>
                  <div class="text-sm text-gray-500">
                    {{ new Date(dir.createdAt).toLocaleString('zh-CN') }}
                  </div>
                </div>
              </div>
              <el-icon class="text-gray-400 ml-2">
                <Promotion />
              </el-icon>
            </div>
          </div>
        </ElCard>
      </div>

      <!-- Quick Start Guide -->
      <div class="mt-16 max-w-3xl mx-auto">
        <ElCard>
          <template #header>
            <h2 class="text-2xl font-bold text-center">快速开始</h2>
          </template>
          <div class="space-y-4">
            <div class="flex items-start gap-4">
              <div
                class="flex-shrink-0 w-8 h-8 bg-primary-500 text-white rounded-full flex items-center justify-center font-bold"
              >
                1
              </div>
              <div>
                <h4 class="font-semibold mb-1">选择工作目录</h4>
                <p class="text-gray-600 dark:text-gray-400">打开您的项目文件夹开始工作</p>
              </div>
            </div>
            <div class="flex items-start gap-4">
              <div
                class="flex-shrink-0 w-8 h-8 bg-primary-500 text-white rounded-full flex items-center justify-center font-bold"
              >
                2
              </div>
              <div>
                <h4 class="font-semibold mb-1">与 AI 对话</h4>
                <p class="text-gray-600 dark:text-gray-400">描述您的需求，AI 将帮助您生成代码</p>
              </div>
            </div>
            <div class="flex items-start gap-4">
              <div
                class="flex-shrink-0 w-8 h-8 bg-primary-500 text-white rounded-full flex items-center justify-center font-bold"
              >
                3
              </div>
              <div>
                <h4 class="font-semibold mb-1">编辑和运行</h4>
                <p class="text-gray-600 dark:text-gray-400">在编辑器中修改代码并在终端中运行</p>
              </div>
            </div>
          </div>
        </ElCard>
      </div>
    </div>
  </div>
</template>

<style scoped>
.home-page {
  min-height: 100vh;
}
</style>
