<script setup lang="ts">
import { ElForm, ElFormItem, ElInput, ElButton } from 'element-plus';
import { computed } from 'vue';
import {saveSettings as saveTauriSettings } from '@/services/tauri/commands';
import { useAppStore } from '@/stores';
import { showSuccess, showError } from '@/utils/toast';

const appStore = useAppStore();

// CLI tool paths
const cliPaths = computed(() => appStore.settings.paths);

// Save settings
async function saveSettings() {
  try {
    await saveTauriSettings(JSON.stringify(appStore.settings));
    showSuccess('CLI 工具路径已保存');
  } catch (error) {
    console.error('Failed to save CLI paths:', error);
    showError('保存失败: ' + (error as Error).message);
  }
}
</script>

<template>
  <div class="cli-paths-settings">
    <div class="settings-header">
      <h2 class="text-2xl font-bold">CLI 工具路径设置</h2>
      <p class="text-gray-500 mt-2">配置常用命令行工具的路径</p>
    </div>

    <div class="settings-content bg-surface rounded-lg border border-border p-6 shadow-sm mt-6">
      <ElForm label-width="120px">
        <ElFormItem label="Node.js">
          <ElInput v-model="cliPaths.nodejs" placeholder="例: /usr/bin/node 或 node" />
          <div class="text-sm text-gray-500 mt-1">Node.js 可执行文件路径</div>
        </ElFormItem>

        <ElFormItem label="Python">
          <ElInput v-model="cliPaths.python" placeholder="例: /usr/bin/python3 或 python" />
          <div class="text-sm text-gray-500 mt-1">Python 可执行文件路径</div>
        </ElFormItem>

        <ElFormItem label="Git">
          <ElInput v-model="cliPaths.git" placeholder="例: /usr/bin/git 或 git" />
          <div class="text-sm text-gray-500 mt-1">Git 可执行文件路径</div>
        </ElFormItem>
      </ElForm>
    </div>

    <!-- Action Buttons -->
    <div class="flex items-center justify-end space-x-4 mt-6">
      <!-- <ElButton @click="resetSettings"> 恢复默认 </ElButton> -->
      <ElButton type="primary" @click="saveSettings"> 保存设置 </ElButton>
    </div>
  </div>
</template>

<style scoped>
.cli-paths-settings {
  padding: 24px;
  max-width: 1200px;
  margin: 0 auto;
}

.settings-header {
  border-bottom: 1px solid var(--color-border);
  padding-bottom: 16px;
}

:deep(.el-form-item) {
  margin-bottom: 24px;
}
</style>
