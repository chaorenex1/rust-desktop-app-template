import { defineStore } from 'pinia';
import { ref } from 'vue';

import {
  sendChatMessageStreaming,
  loadChatSessions,
  deleteChatSession,
  updateChatSessionName,
} from '@/services/tauri/commands';
import type { ChatMessage, ChatSession, SendMessageOptions, AiResponseEventPayload } from '@/utils/types';

// type RawChatSession = ChatSession &
//   Partial<{
//     created_at: string;
//     updated_at: string;
//     message_count: number;
//     first_message_preview: string;
//     sessionId: string;
//   }>;

// function normalizeSession(session: RawChatSession): ChatSession {
//   return {
//     ...session,
//     createdAt: session.createdAt || session.created_at || '',
//     updatedAt: session.updatedAt || session.updated_at || '',
//     sessionId: session.sessionId || undefined,
//     messageCount:
//       session.messageCount ??
//       session.message_count ??
//       session.messages?.length ??
//       0,
//     firstMessagePreview: session.firstMessagePreview || session.first_message_preview || '',
//   };
// }

export const useChatStore = defineStore('chat', () => {
  const messages = ref<ChatMessage[]>([]);
  const associatedFiles = ref<string[]>([]);
  const currentSessionId = ref<string>('');
  const currentRequestId = ref<string>('');
  const currentCodeCli = ref<string>('');
  const associatedClipboardImage = ref<string>('');
  const isStreaming = ref(false);
  const sessions = ref<ChatSession[]>([]);
  const isSessionsLoading = ref(false);
  const codeCliChanged = ref(false);

  function setAssociatedFiles(paths: string[]) {
    associatedFiles.value = [...paths];
  }

  function removeAssociatedFile(index: number) {
    associatedFiles.value.splice(index, 1);
  }

  function getCurrentSessionId(): string {
    return currentSessionId.value;
  }

  function getCurrentCodeCli(): string {
    return currentCodeCli.value;
  }

  function setCurrentCodeCli(codeCli: string) {
    currentCodeCli.value = codeCli;
  }

  async function sendMessage(options: SendMessageOptions): Promise<void> {
    const content = options.content.trim();
    if (!content) {
      return;
    }

    const files = [...options.files];
    const timestamp = new Date().toISOString();
    const model = options.model;
    const codeCli = options.codeCli;
    if(currentCodeCli.value){
      currentCodeCli.value = codeCli;
    }else if(codeCli != currentCodeCli.value){
      codeCliChanged.value = true;
    }else{
      codeCliChanged.value = false;
    }

    messages.value.push({
      id: `${Date.now()}-user`,
      role: 'user',
      content,
      timestamp,
      files,
      model,
      sessionId: currentSessionId.value || '',
      workspaceId: options.workspaceId,
    });

    try {
      isStreaming.value = true;
      const requestId = await sendChatMessageStreaming(
        content,
        files,
        options.codeCli,
        model,
        currentSessionId.value || '',
        options.workspaceId,
        options.workspaceDir,
        codeCliChanged.value
      );
      currentRequestId.value = requestId;
      messages.value.push({
        id: requestId,
        role: 'assistant',
        content: '',
        timestamp: new Date().toISOString(),
        files,
        model,
        sessionId: currentSessionId.value || '',
        workspaceId: options.workspaceId,
      });
    } catch (error) {
      isStreaming.value = false;
      currentRequestId.value = '';
      console.error('Failed to send message:', error);
      throw error;
    }
  }

  function handleAiResponse(payload: AiResponseEventPayload): void {
    if (!payload?.request_id) {
      return;
    }
    const targetIndex = messages.value.findIndex((msg) => msg.id === payload.request_id);
    if (targetIndex === -1) {
      return;
    }

    const message = messages.value[targetIndex];
    if (message) {
      message.content += payload.delta || '';
    }

    if (payload.done) {
      void finalizeStreaming(payload.session_id || '');
    }
  }

  async function finalizeStreaming(newSessionId: string) {
    isStreaming.value = false;
    currentRequestId.value = '';
    if (newSessionId) {
      currentSessionId.value = newSessionId as string;
    }
    // await autoSaveChatSession();
  }

  // async function autoSaveChatSession() {
  //   if (!messages.value.length) {
  //     return;
  //   }

  //   try {
  //     const session = await saveChatSession(
  //       currentSessionId.value,
  //       null,
  //       messages.value,
  //       currentSessionId.value
  //     );
  //     currentSessionId.value = session.id;
  //   } catch (error) {
  //     console.error('Failed to auto-save chat session:', error);
  //   }
  // }

  function loadSessionFromHistory(session: ChatSession) {
    messages.value = [...session.messages];
    currentSessionId.value = session.id;
    associatedFiles.value = [];
  }

  function clearChat() {
    messages.value = [];
    associatedFiles.value = [];
    currentSessionId.value = '';
    currentRequestId.value = '';
    isStreaming.value = false;
  }

  async function fetchSessions(workspaceId: string, limit = 50): Promise<ChatSession[]> {
    isSessionsLoading.value = true;
    try {
      sessions.value = await loadChatSessions(workspaceId, limit);
      return sessions.value;
    } finally {
      isSessionsLoading.value = false;
    }
  }

  async function removeSession(sessionId: string): Promise<void> {
    await deleteChatSession(sessionId);
    sessions.value = sessions.value.filter((session) => session.id !== sessionId);

    if (currentSessionId.value === sessionId) {
      currentSessionId.value = '';
    }
  }

  async function renameSession(sessionId: string, name: string): Promise<ChatSession> {
    const updated = await updateChatSessionName(sessionId, name);
    const index = sessions.value.findIndex((session) => session.id === sessionId);
    if (index !== -1) {
      sessions.value[index] = updated;
    }
    if (currentSessionId.value === sessionId) {
      currentSessionId.value = updated.id;
    }
    return updated;
  }

  return {
    // State
    messages,
    associatedFiles,
    currentSessionId,
    currentRequestId,
    isStreaming,
    sessions,
    isSessionsLoading,

    // Actions
    getCurrentSessionId,
    getCurrentCodeCli,
    setCurrentCodeCli,
    setAssociatedFiles,
    removeAssociatedFile,
    sendMessage,
    handleAiResponse,
    loadSessionFromHistory,
    clearChat,
    fetchSessions,
    removeSession,
    renameSession,
  };
});
