<script setup lang="ts">
import { ref, computed } from 'vue';
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
import { useAppStore } from '@/stores';
import { showSuccess } from '@/utils/toast';

const router = useRouter();
const appStore = useAppStore();
const settings = computed(() => appStore.settings);
const themes = [
  { label: '浅色', value: 'light' },
  { label: '深色', value: 'dark' },
]

function handleSave() {
  appStore.saveSettings();
  showSuccess('设置已保存');
}

function handleReset() {
  appStore.resetToDefaults();
  showSuccess('设置已重置为默认值');
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
            <h2 class="text-2xl font-bold">设置</h2>
            <p class="text-sm text-gray-500 mt-2">应用设置</p>
          </div>
          <button type="button" class="back-button" @click="goBack">返回</button>
        </div>
        <div class="max-w-3xl wx-auto space-y-6 settings-page-content">
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
          </ElForm>
        </ElCard>

        <!-- 编辑器设置 -->
        <ElCard>
          <template #header>
            <h2 class="text-xl font-semibold">编辑器</h2>
          </template>
          <ElForm label-width="120px">
            <ElFormItem label="自动保存">
              <ElSwitch v-model="settings.editor.autoSave" />
            </ElFormItem>
            <ElFormItem label="字体大小">
              <ElInput v-model.number="settings.editor.fontSize" type="number" :min="10" :max="24">
                <template #append>px</template>
              </ElInput>
            </ElFormItem>
          </ElForm>
        </ElCard>
        <!--聊天设置-->
        <ElCard>
          <template #header>
            <h2 class="text-xl font-semibold">编辑器</h2>
          </template>
          <ElForm label-width="120px">
            <ElFormItem label="ENTER发送">
              <ElSwitch v-model="settings.chat.sendWithEnter" />
            </ElFormItem>
            <ElFormItem label="Shift+Enter发送">
              <ElSwitch v-model="settings.chat.switchLineWithShiftEnter" />
            </ElFormItem>
          </ElForm>
        </ElCard>
        <!-- 终端设置 -->
        <ElCard>
          <template #header>
            <h2 class="text-xl font-semibold">终端</h2>
          </template>
          <ElForm label-width="120px">
            <ElFormItem label="字体大小">
              <ElInput v-model.number="settings.terminal.fontSize" type="number" :min="10" :max="24">
                <template #append>px</template>
              </ElInput>
            </ElFormItem>
            <ElFormItem label="字体家族">
              <ElInput v-model="settings.terminal.fontFamily" placeholder="输入字体家族" />
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
