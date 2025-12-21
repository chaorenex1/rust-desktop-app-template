<script setup lang="ts">
import { ref, computed, onMounted, watch } from 'vue';
import { ElDialog, ElInput, ElButton, ElIcon, ElMessageBox, ElEmpty } from 'element-plus';
import { Search, Delete, Edit, Loading, Clock } from '@element-plus/icons-vue';

import { loadChatSessions, deleteChatSession, updateChatSessionName } from '@/services/tauri/commands';
import type { ChatSession } from '@/utils/types';
import { showSuccess, showError } from '@/utils/toast';

const props = defineProps<{
  modelValue: boolean;
}>();

const emit = defineEmits<{
  (e: 'update:modelValue', value: boolean): void;
  (e: 'load-session', session: ChatSession): void;
}>();

type RawChatSession = ChatSession &
  Partial<{
    created_at: string;
    updated_at: string;
    message_count: number;
    first_message_preview: string;
  }>;

const TIMESTAMP_FRACTION_REGEX = /\.(\d{3})\d+/;

function parseTimestamp(value?: string | null): Date | null {
  if (!value) return null;
  const normalized = value.replace(TIMESTAMP_FRACTION_REGEX, '.$1');
  const date = new Date(normalized);
  return Number.isNaN(date.getTime()) ? null : date;
}

function formatTimestamp(
  value?: string | null,
  type: 'datetime' | 'time' = 'datetime',
  options?: Intl.DateTimeFormatOptions
): string {
  const date = parseTimestamp(value);
  if (!date) return value || '--';
  if (type === 'time') {
    return date.toLocaleTimeString('zh-CN', options);
  }
  return date.toLocaleString('zh-CN', options);
}

function normalizeSession(session: RawChatSession): ChatSession {
  return {
    ...session,
    createdAt: session.createdAt || session.created_at || '',
    updatedAt: session.updatedAt || session.updated_at || '',
    messageCount:
      session.messageCount ??
      session.message_count ??
      session.messages?.length ??
      0,
    firstMessagePreview: session.firstMessagePreview || session.first_message_preview || '',
  };
}

// 状态
const searchQuery = ref('');
const sessions = ref<ChatSession[]>([]);
const loading = ref(false);
const selectedSession = ref<ChatSession | null>(null);
const editingSessionId = ref<string | null>(null);
const editingName = ref('');

// 计算属性
const filteredSessions = computed(() => {
  const query = searchQuery.value.trim().toLowerCase();
  if (!query) return sessions.value;

  return sessions.value.filter((session) => {
    const nameMatch = session.name?.toLowerCase().includes(query);
    const previewMatch = session.firstMessagePreview.toLowerCase().includes(query);
    return nameMatch || previewMatch;
  });
});

// 方法
async function loadSessions() {
  loading.value = true;
  try {
    const result = await loadChatSessions(50); // 默认加载50个
    sessions.value = result.map((session) => normalizeSession(session as RawChatSession));
    console.log(`Loaded ${result.length} chat sessions`);
  } catch (error) {
    console.error('Failed to load chat sessions:', error);
    showError('加载会话历史失败');
  } finally {
    loading.value = false;
  }
}

function selectSession(session: ChatSession) {
  selectedSession.value = session;
}

// function formatDate(dateStr: string): string {
//   try {
//     const date = new Date(dateStr);
//     const now = new Date();
//     const diffMs = now.getTime() - date.getTime();
//     const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

//     if (diffDays === 0) {
//       return date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
//     } else if (diffDays === 1) {
//       return '昨天 ' + date.toLocaleTimeString('zh-CN', { hour: '2-digit', minute: '2-digit' });
//     } else if (diffDays < 7) {
//       return `${diffDays}天前`;
//     } else {
//       return date.toLocaleDateString('zh-CN', { month: '2-digit', day: '2-digit' });
//     }
//   } catch {
//     return dateStr;
//   }
// }

function loadSession(session: ChatSession) {
  emit('load-session', session);
  emit('update:modelValue', false);
  showSuccess('会话已加载');
}

async function deleteSession(session: ChatSession, event: Event) {
  event.stopPropagation();

  try {
    await ElMessageBox.confirm(`确定要删除会话"${session.name || '未命名会话'}"吗？`, '确认删除', {
      confirmButtonText: '删除',
      cancelButtonText: '取消',
      type: 'warning',
    });

    await deleteChatSession(session.id);
    sessions.value = sessions.value.filter((s) => s.id !== session.id);

    if (selectedSession.value?.id === session.id) {
      selectedSession.value = null;
    }

    showSuccess('会话已删除');
  } catch (error: any) {
    if (error !== 'cancel') {
      console.error('Failed to delete session:', error);
      showError('删除会话失败');
    }
  }
}

function startEditing(session: ChatSession, event: Event) {
  event.stopPropagation();
  editingSessionId.value = session.id;
  editingName.value = session.name || '';
}

async function saveSessionName(session: ChatSession) {
  if (!editingName.value.trim()) {
    showError('会话名称不能为空');
    return;
  }

  try {
    const updated = normalizeSession(
      (await updateChatSessionName(session.id, editingName.value.trim())) as RawChatSession
    );
    const index = sessions.value.findIndex((s) => s.id === session.id);
    if (index !== -1) {
      sessions.value[index] = updated;
    }
    if (selectedSession.value?.id === session.id) {
      selectedSession.value = updated;
    }
    editingSessionId.value = null;
    showSuccess('会话名称已更新');
  } catch (error) {
    console.error('Failed to update session name:', error);
    showError('更新会话名称失败');
  }
}

function cancelEditing() {
  editingSessionId.value = null;
  editingName.value = '';
}

onMounted(() => {
  loadSessions();
});

watch(
  () => props.modelValue,
  (isOpen) => {
    if (isOpen) {
      loadSessions();
    }
  }
);
</script>

<template>
  <ElDialog
    :model-value="props.modelValue"
    @update:model-value="emit('update:modelValue', $event)"
    title="聊天历史"
    width="80%"
    :close-on-click-modal="false"
  >
    <!-- 搜索栏 -->
    <div class="mb-3">
      <ElInput
        v-model="searchQuery"
        :prefix-icon="Search"
        size="default"
        placeholder="搜索会话名称或内容..."
        clearable
      />
    </div>

    <!-- 主内容区 -->
    <div class="flex h-[500px] border border-border rounded">
      <!-- 左侧：会话列表 -->
      <div class="w-2/5 border-r border-border overflow-auto">
        <div v-if="loading" class="flex items-center justify-center h-full text-text-secondary">
          <ElIcon :size="32" class="animate-spin">
            <Loading />
          </ElIcon>
        </div>

        <div v-else-if="filteredSessions.length === 0" class="flex items-center justify-center h-full">
          <ElEmpty description="暂无会话历史" />
        </div>

        <div v-else class="space-y-1 p-2">
          <div
            v-for="session in filteredSessions"
            :key="session.id"
            class="p-3 rounded cursor-pointer hover:bg-surface transition-colors"
            :class="{ 'bg-primary/10 border-l-2 border-primary': selectedSession?.id === session.id }"
            @click="selectSession(session)"
          >
            <!-- 会话名称/重命名 -->
            <div v-if="editingSessionId === session.id" class="mb-2" @click.stop>
              <ElInput
                v-model="editingName"
                size="small"
                placeholder="输入会话名称"
                @keyup.enter="saveSessionName(session)"
                @keyup.esc="cancelEditing"
              >
                <template #append>
                  <div class="flex space-x-1 min-w-[120px] justify-end">
                    <ElButton size="small" @click="saveSessionName(session)">保存</ElButton>
                    <ElButton size="small" @click="cancelEditing">取消</ElButton>
                  </div>
                </template>
              </ElInput>
            </div>
            <div v-else class="flex items-center justify-between mb-1">
              <div class="flex-1 font-medium text-sm truncate" :title="session.name || '未命名会话'">
                {{ session.name || '未命名会话' }}
              </div>
              <div class="flex items-center space-x-1">
                <ElButton
                  :icon="Edit"
                  size="small"
                  text
                  @click="startEditing(session, $event)"
                  title="重命名"
                />
                <ElButton
                  :icon="Delete"
                  size="small"
                  text
                  type="danger"
                  @click="deleteSession(session, $event)"
                  title="删除"
                />
              </div>
            </div>

            <!-- 会话信息 -->
            <div class="flex items-center text-xs text-text-secondary space-x-2 mb-1">
              <span>{{ session.messageCount }} 条消息</span>
              <span>·</span>
              <span>{{ formatTimestamp(session.updatedAt) }}</span>
            </div>

            <!-- 首条消息预览 -->
            <div class="text-xs text-text-secondary line-clamp-2" :title="session.firstMessagePreview">
              {{ session.firstMessagePreview || '(空消息)' }}
            </div>
          </div>
        </div>
      </div>

      <!-- 右侧：预览面板 -->
      <div class="w-3/5 flex flex-col">
        <div v-if="!selectedSession" class="flex items-center justify-center h-full text-text-secondary">
          <div class="text-center">
            <ElIcon :size="48" class="mb-2">
              <Clock />
            </ElIcon>
            <div>选择一个会话以查看详情</div>
          </div>
        </div>

        <div v-else class="flex flex-col h-full">
          <!-- 预览头部 -->
          <div class="border-b border-border p-3">
            <div class="font-semibold text-base mb-1">
              {{ selectedSession.name || '未命名会话' }}
            </div>
            <div class="text-xs text-text-secondary">
              创建于 {{ formatTimestamp(selectedSession.createdAt) }} · 更新于
              {{ formatTimestamp(selectedSession.updatedAt) }} · {{ selectedSession.messageCount }} 条消息
            </div>
          </div>

          <!-- 消息预览 -->
          <div class="flex-1 overflow-auto p-4 space-y-3 bg-surface/30">
            <div
              v-for="msg in selectedSession.messages"
              :key="msg.id"
              class="flex"
              :class="msg.role === 'user' ? 'justify-end' : 'justify-start'"
            >
              <div
                class="max-w-[80%] rounded-lg px-3 py-2 text-sm shadow-sm"
                :class="msg.role === 'user' ? 'bg-primary text-white' : 'bg-surface text-text'"
              >
                <div class="whitespace-pre-wrap break-words">
                  {{ msg.content }}
                </div>
                <div class="mt-1 text-[11px] opacity-70 text-right">
                  {{
                    formatTimestamp(
                      msg.timestamp,
                      'time',
                      {
                        hour: '2-digit',
                        minute: '2-digit',
                      }
                    )
                  }}
                </div>
              </div>
            </div>
          </div>

          <!-- 操作按钮 -->
          <div class="border-t border-border p-3 flex justify-end space-x-2">
            <ElButton @click="emit('update:modelValue', false)">关闭</ElButton>
            <ElButton type="primary" @click="loadSession(selectedSession)">加载此会话</ElButton>
          </div>
        </div>
      </div>
    </div>
  </ElDialog>
</template>

<style scoped>
.line-clamp-2 {
  display: -webkit-box;
  -webkit-line-clamp: 2;
  -webkit-box-orient: vertical;
  overflow: hidden;
}

.animate-spin {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}
</style>
