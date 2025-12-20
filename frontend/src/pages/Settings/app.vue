<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import {
  ElButton,
  ElCard,
  ElForm,
  ElFormItem,
  ElInput,
  ElSwitch,
  ElSelect,
  ElOption,
  ElMessage,
  ElPageHeader,
} from 'element-plus';
import { useAppStore } from '../../stores/appStore';

const router = useRouter();
const appStore = useAppStore();

const settings = ref({
  theme: 'light',
  language: 'zh-CN',
  fontSize: 14,
  autoSave: true,
  aiModel: 'gpt-3.5-turbo',
  apiKey: '',
});

const themes = [
  { label: '浅色', value: 'light' },
  { label: '深色', value: 'dark' },
  { label: '自动', value: 'auto' },
];

const languages = [
  { label: '简体中文', value: 'zh-CN' },
  { label: 'English', value: 'en' },
];

const aiModels = [
  { label: 'GPT-3.5 Turbo', value: 'gpt-3.5-turbo' },
  { label: 'GPT-4', value: 'gpt-4' },
  { label: 'GPT-4 Turbo', value: 'gpt-4-turbo' },
];

function handleSave() {
  // TODO: Implement save settings logic
  ElMessage.success('设置已保存');
}

function handleReset() {
  // Reset to default settings
  settings.value = {
    theme: 'light',
    language: 'zh-CN',
    fontSize: 14,
    autoSave: true,
    aiModel: 'gpt-3.5-turbo',
    apiKey: '',
  };
  ElMessage.info('设置已重置为默认值');
}

function goBack() {
  router.push('/dashboard');
}
</script>

<template>
  <div class="settings-page min-h-screen bg-gray-50 dark:bg-gray-900">
    <div class="container mx-auto">
      <div class="settings-page-wrapper">
        <div class="settings-page-header">
          <div class="settings-page-header-main">
            <h2 class="text-2xl font-bold">AI 模型管理</h2>
            <p class="text-sm text-gray-500 mt-2">配置和管理 AI 模型设置</p>
          </div>
          <button type="button" class="back-button" @click="goBack">返回</button>
        </div>
        <div class="max-w-3xl space-y-6 settings-page-content">
        <!-- 外观设置 -->
        <ElCard>
          <template #header>
            <h2 class="text-xl font-semibold">外观</h2>
          </template>
          <ElForm label-width="120px">
            <ElFormItem label="主题">
              <ElSelect v-model="settings.theme" class="w-full">
                <ElOption
                  v-for="theme in themes"
                  :key="theme.value"
                  :label="theme.label"
                  :value="theme.value"
                />
              </ElSelect>
            </ElFormItem>
            <ElFormItem label="语言">
              <ElSelect v-model="settings.language" class="w-full">
                <ElOption
                  v-for="lang in languages"
                  :key="lang.value"
                  :label="lang.label"
                  :value="lang.value"
                />
              </ElSelect>
            </ElFormItem>
            <ElFormItem label="字体大小">
              <ElInput v-model.number="settings.fontSize" type="number" :min="10" :max="24">
                <template #append>px</template>
              </ElInput>
            </ElFormItem>
          </ElForm>
        </ElCard>

        <!-- 编辑器设置 -->
        <ElCard>
          <template #header>
            <h2 class="text-xl font-semibold">编辑器</h2>
          </template>
          <ElForm label-width="120px">
            <ElFormItem label="自动保存">
              <ElSwitch v-model="settings.autoSave" />
            </ElFormItem>
          </ElForm>
        </ElCard>
        <!-- 操作按钮 -->
        <div class="flex justify-end gap-4">
          <ElButton @click="handleReset">重置为默认</ElButton>
          <ElButton type="primary" @click="handleSave">保存设置</ElButton>
        </div>
      </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.settings-page-wrapper {
  display: flex;
  flex-direction: column;
  height: 100%;
  padding: 24px 32px;
}

.settings-page-header {
  margin-bottom: 24px;
  padding-bottom: 16px;
  border-bottom: 1px solid var(--color-border);
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
}

.settings-page-header-main {
  flex: 1;
}

.settings-page-content {
  flex: 1;
  overflow-y: auto;
}

.back-button {
  padding: 6px 14px;
  font-size: 14px;
  border-radius: 999px;
  border: 1px solid var(--color-border);
  background-color: transparent;
  color: var(--color-text-secondary);
  cursor: pointer;
  white-space: nowrap;
  transition: all 0.15s ease-in-out;
}

.back-button:hover {
  background-color: rgba(0, 0, 0, 0.04);
  color: var(--color-text);
}
</style>
