import { listen } from '@tauri-apps/api/event';
import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow';

import type { useAppStore } from '@/stores/appStore';

type LightweightModePayload = {
  enabled: boolean;
  reason?: string;
};

export async function initLightweightModeMonitor(appStore: ReturnType<typeof useAppStore>) {
  const win = getCurrentWebviewWindow();

  const updateFromWindowState = async (reason: string) => {
    try {
      const [isMinimized, isVisible] = await Promise.all([win.isMinimized(), win.isVisible()]);
      const enabled = Boolean(isMinimized) || !Boolean(isVisible);
      appStore.setLightweightMode(enabled, reason);
    } catch (e) {
      // If the platform doesn't support the query, fall back to visibility.
      const enabled = typeof document !== 'undefined' ? document.hidden : false;
      appStore.setLightweightMode(enabled, reason);
    }
  };

  // 1) Backend-driven events (tray show/hide, close-to-tray)
  const unlistenBackend = await listen<LightweightModePayload>('app:lightweight-mode', (event) => {
    appStore.setLightweightMode(Boolean(event.payload?.enabled), event.payload?.reason);
  });

  // 2) Frontend window state detection (minimize/restore)
  const unlistenResized = await win.onResized(() => {
    void updateFromWindowState('resized');
  });

  const unlistenFocus = await win.onFocusChanged(() => {
    void updateFromWindowState('focus');
  });

  const onVisibilityChange = () => {
    void updateFromWindowState('visibilitychange');
  };
  document.addEventListener('visibilitychange', onVisibilityChange);

  // Initial state
  await updateFromWindowState('init');

  return () => {
    try {
      unlistenBackend();
      unlistenResized();
      unlistenFocus();
      document.removeEventListener('visibilitychange', onVisibilityChange);
    } catch {
      // no-op
    }
  };
}
