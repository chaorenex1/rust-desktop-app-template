import type { useAppStore } from '@/stores';
import { useChatStore } from '@/stores/chatStore';
import type { AiResponseEventPayload } from '@/utils/types';

import { eventService } from './events';

interface ListenerContext {
  appStore: ReturnType<typeof useAppStore>;
  chatStore: ReturnType<typeof useChatStore>;
}

type CleanupFn = () => void;

let cleanupFns: CleanupFn[] = [];

export async function initTauriEventListeners(context: ListenerContext): Promise<CleanupFn> {
  await eventService.initialize();
  disposeTauriEventListeners();

  cleanupFns = [
    eventService.subscribe('ai-response', (payload) => {
      try {
        const parsed = parseAiResponsePayload(payload);
        if (parsed) {
          context.chatStore.handleAiResponse(parsed);
          context.appStore.setCurrentSessionId(parsed.session_id || '');
        }
      } catch (error) {
        console.error('Failed to handle ai-response event:', error);
      }
    }),
    eventService.subscribe('settings:updated', () => {
      void context.appStore.loadSettings();
    }),
  ];

  return () => {
    disposeTauriEventListeners();
  };
}

export function disposeTauriEventListeners() {
  if (!cleanupFns.length) {
    return;
  }

  cleanupFns.forEach((cleanup) => {
    try {
      cleanup();
    } catch (error) {
      console.warn('Failed to dispose event listener:', error);
    }
  });
  cleanupFns = [];
}

function parseAiResponsePayload(payload: unknown): AiResponseEventPayload | null {
  if (!payload) {
    return null;
  }

  if (typeof payload === 'string') {
    try {
      return JSON.parse(payload) as AiResponseEventPayload;
    } catch (error) {
      console.error('Failed to parse ai-response payload:', error);
      return null;
    }
  }

  if (typeof payload === 'object') {
    return payload as AiResponseEventPayload;
  }

  return null;
}
