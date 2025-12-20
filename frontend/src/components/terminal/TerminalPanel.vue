<script setup lang="ts">
import { ref, onMounted, onUnmounted, nextTick, computed } from 'vue';
import { Terminal } from '@xterm/xterm';
import { FitAddon } from '@xterm/addon-fit';
import { WebLinksAddon } from '@xterm/addon-web-links';
import '@xterm/xterm/css/xterm.css';
import { Plus, Refresh } from '@element-plus/icons-vue';
import { ElTabs, ElTabPane, ElButton, ElSelect, ElOption } from 'element-plus';
import type { TabsPaneContext, TabPaneName } from 'element-plus';

import { useAppStore, useTerminalStore } from '@/stores';
import type { TerminalTab } from '@/utils/types';
import { spawnTerminal, killTerminal, executeTerminalCommand } from '@/services/tauri/commands';

const appStore = useAppStore();
const terminalStore = useTerminalStore();
const isWindows = navigator.userAgent.includes('Windows');
const shellOptions = computed(() =>
  isWindows
    ? [
        { label: 'PowerShell', value: 'powershell' },
        { label: 'cmd', value: 'cmd' },
      ]
    : [
        { label: 'bash', value: 'bash' },
        { label: 'zsh', value: 'zsh' },
      ]
);
const terminalContainer = ref<HTMLElement>();
// 仅在组件中保存 xterm 实例映射，真正的数据（id/name/sessionId、当前激活索引）放到 Pinia store 中
const terminalInstances = computed(() => terminalStore.terminalInstances);
const terminals = computed(() => terminalStore.terminals);
const activeTerminalIndex = computed<TabPaneName>({
  get() {
    return terminalStore.activeIndex;
  },
  set(value: TabPaneName) {
    const idx = typeof value === 'number' ? value : Number(value);
    if (!Number.isNaN(idx)) {
      terminalStore.setActiveIndex(idx);
    }
  },
});

// Initialize terminal
onMounted(() => {
  createNewTerminal();
});

// Cleanup on unmount
onUnmounted(() => {
  terminalStore.clear();
});

// Create new terminal (with backend session)
async function createNewTerminal() {
  const name = `终端 ${terminalStore.terminals.length + 1}`;

  // Create backend terminal session
  let sessionId: string;
  try {
    sessionId = await spawnTerminal('.');
  } catch (error) {
    console.error('Failed to spawn terminal session:', error);
    return;
  }

  // Create terminal instance
  const terminal = new Terminal({
    fontSize: appStore.settings.terminal.fontSize,
    fontFamily: appStore.settings.terminal.fontFamily,
    theme: {
      background: getComputedStyle(document.documentElement)
        .getPropertyValue('--color-background')
        .trim(),
      foreground: getComputedStyle(document.documentElement)
        .getPropertyValue('--color-text')
        .trim(),
      cursor: getComputedStyle(document.documentElement).getPropertyValue('--color-primary').trim(),
    },
    cursorBlink: true,
    scrollback: 1000,
  });

  // Add addons
  const fitAddonInstance = new FitAddon();
  const webLinksAddon = new WebLinksAddon();

  terminal.loadAddon(fitAddonInstance);
  terminal.loadAddon(webLinksAddon);

  // Store terminal metadata and keep xterm instance locally
  terminalStore.addTerminal({ id: sessionId, name, sessionId, terminal });
  if (isWindows) {
    appStore.setCurrentShell('powershell');
  } else {
    appStore.setCurrentShell('bash');
  }

  // Initialize terminal after DOM update
  nextTick(() => {
    const container = terminalContainer.value;
    if (container) {
      terminal.open(container);
      fitAddonInstance.fit();

      // Write welcome message
      terminal.writeln('\x1b[1;32m欢迎使用 Code AI Assistant 终端\x1b[0m');
      terminal.writeln('输入 "help" 查看可用命令');
      terminal.writeln('');

      // Set up command handling bound to this backend session
      setupCommandHandling(terminal, sessionId);
    }
  });
}

// Set up command handling（带简单历史）
function setupCommandHandling(terminal: Terminal, sessionId: string) {
  let command = '';
  const history: string[] = [];
  let historyIndex = -1;

  const prompt = () => {
    terminal.write('$ ');
  };

  const eraseCurrentInput = () => {
    for (let i = 0; i < command.length; i += 1) {
      terminal.write('\b \b');
    }
  };

  terminal.onKey(async ({ key, domEvent }) => {
    const printable =
      !domEvent.altKey && !domEvent.ctrlKey && !domEvent.metaKey && domEvent.key.length === 1;

    switch (domEvent.key) {
      case 'Enter': {
        terminal.writeln('');
        await handleCommand(command, terminal, sessionId);
        if (command.trim()) {
          history.push(command);
          historyIndex = history.length;
        }
        command = '';
        prompt();
        break;
      }
      case 'Backspace': {
        if (command.length > 0) {
          command = command.slice(0, -1);
          terminal.write('\b \b');
        }
        break;
      }
      case 'ArrowUp': {
        if (history.length === 0) break;
        if (historyIndex > 0) {
          historyIndex -= 1;
        } else {
          historyIndex = 0;
        }
        eraseCurrentInput();
        command = history[historyIndex] ?? '';
        terminal.write(command);
        break;
      }
      case 'ArrowDown': {
        if (history.length === 0) break;
        if (historyIndex < history.length - 1) {
          historyIndex += 1;
          command = history[historyIndex] ?? '';
        } else {
          historyIndex = history.length;
          command = '';
        }
        eraseCurrentInput();
        terminal.write(command);
        break;
      }
      default: {
        if (domEvent.ctrlKey && (domEvent.key === 'c' || domEvent.key === 'C')) {
          terminal.writeln('^C');
          command = '';
          prompt();
        } else if (printable) {
          command += key;
          terminal.write(key);
        }
        break;
      }
    }
  });

  // Initial prompt
  prompt();
}

// Handle command execution
async function handleCommand(command: string, terminal: Terminal, sessionId: string) {
  const trimmedCommand = command.trim();

  if (!trimmedCommand) {
    return;
  }

  // Built-in commands
  if (trimmedCommand === 'help') {
    terminal.writeln('可用命令:');
    terminal.writeln('  help     - 显示此帮助信息');
    terminal.writeln('  clear    - 清空终端');
    terminal.writeln('  ls       - 列出文件');
    terminal.writeln('  pwd      - 显示当前目录');
    terminal.writeln('  echo     - 显示文本');
    terminal.writeln('  exit     - 退出终端');
    return;
  }

  if (trimmedCommand === 'clear') {
    terminal.clear();
    terminal.write('$ ');
    return;
  }

  if (trimmedCommand === 'exit') {
    const index = terminals.value.findIndex(
      (t: TerminalTab) => terminalInstances.value[t.id] === terminal
    );
    if (index >= 0) {
      closeTerminal(index);
    }
    return;
  }

  // Execute command via backend terminal session
  try {
    const shell = appStore.currentShell;
    const result = await executeTerminalCommand(sessionId as string, shell, trimmedCommand);

    if (result) {
      terminal.writeln(result);
    }
  } catch (error) {
    terminal.writeln(`\x1b[1;31m错误: ${error}\x1b[0m`);
  }

  terminal.write('$ ');
}

// Close terminal
function closeTerminal(name: TabPaneName) {
  const index = typeof name === 'number' ? name : Number(name);
  const term = terminals.value[index];
  if (term) {
    // Best-effort attempt to kill backend session
    void killTerminal(term.sessionId).catch((err) => {
      console.error('Failed to kill terminal session:', err);
    });

    const instance = terminalInstances.value[term.id];
    if (instance) {
      instance.dispose();
      delete terminalInstances.value[term.id];
    }

    terminalStore.removeTerminal(index);

    // if (terminalStore.terminals.length === 0) {
    //   createNewTerminal();
    // }
  }
}

// Switch terminal
function switchTerminal(pane: TabsPaneContext) {
  const index = typeof pane.paneName === 'number' ? pane.paneName : Number(pane.paneName);
  if (!Number.isNaN(index)) {
    terminalStore.setActiveIndex(index);
  }
}

// Refresh terminal
function refreshTerminal() {
  const activeIndex = terminalStore.activeIndex;
  const termMeta = terminals.value[activeIndex];
  const terminal = termMeta ? terminalInstances.value[termMeta.id] : undefined;
  if (terminal) {
    terminal.clear();
    terminal.writeln('\x1b[1;32m终端已刷新\x1b[0m');
    terminal.writeln('');
  }
}
</script>

<template>
  <div class="h-full flex flex-col">
    <!-- Terminal Tabs -->
    <div class="border-b border-border bg-surface">
      <div class="flex items-center justify-between px-4 py-2">
        <ElTabs
          v-model="activeTerminalIndex"
          type="card"
          closable
          @tab-click="switchTerminal"
          @tab-remove="closeTerminal"
        >
          <ElTabPane
            v-for="(term, index) in terminals"
            :key="term.id"
            :label="term.name"
            :name="index"
          />
        </ElTabs>

        <div class="flex items-center space-x-2">
          <ElButton :icon="Plus" size="small" text @click="createNewTerminal"> 新建 </ElButton>

          <ElButton :icon="Refresh" size="small" text @click="refreshTerminal"> 刷新 </ElButton>

          <ElSelect
            v-model="appStore.currentShell"
            size="small"
            style="width: 100px"
            @change="appStore.setCurrentShell"
          >
            <ElOption
              v-for="opt in shellOptions"
              :key="opt.value"
              :label="opt.label"
              :value="opt.value"
            />
          </ElSelect>
        </div>
      </div>
    </div>

    <!-- Terminal Area -->
    <div ref="terminalContainer" class="flex-1 overflow-hidden" />

    <!-- Status Bar -->
    <div class="border-t border-border bg-surface px-4 py-1 text-xs text-text-secondary">
      <div class="flex items-center justify-between">
        <div>
          <span>终端: {{ terminals[Number(activeTerminalIndex)]?.name || '' }}</span>
          <span class="mx-2">|</span>
          <span>Shell: {{ appStore.currentShell }}</span>
        </div>

        <div>
          <span>就绪</span>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
:deep(.xterm) {
  padding: 8px;
  height: 100%;
}

:deep(.xterm-screen) {
  height: 100%;
}

:deep(.el-tabs__header) {
  margin: 0;
}

:deep(.el-tabs__nav-wrap) {
  padding: 0;
}
</style>
