import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { TerminalTab } from '../utils/types';
import { Terminal } from '@xterm/xterm';

export const useTerminalStore = defineStore('terminal', () => {
  const terminalInstances = ref<Record<string, Terminal>>({});
  const terminals = ref<TerminalTab[]>([]);
  const activeIndex = ref(0);

  const getActiveTerminal = computed(() => terminals.value[activeIndex.value]);

  const getActiveIndex = computed(() => activeIndex.value);

  const getActiveTerminalInstance = computed(() => {
    const activeTerminal = terminals.value[activeIndex.value];
    return activeTerminal ? terminalInstances.value[activeTerminal.id] : null;
  });

  const getActiveTerminalTab = computed(() => terminals.value[activeIndex.value]);

  function addTerminal(tab: TerminalTab) {
    terminals.value.push(tab);
    activeIndex.value = terminals.value.length - 1;
    terminalInstances.value[tab.id] = tab.terminal;
  }

  function removeTerminal(index: number) {
    if (index < 0 || index >= terminals.value.length) return;
    terminals.value.splice(index, 1);
    if (activeIndex.value >= terminals.value.length) {
      activeIndex.value = terminals.value.length > 0 ? terminals.value.length - 1 : 0;
    }
  }

  function setActiveIndex(index: number) {
    if (index >= 0 && index < terminals.value.length) {
      activeIndex.value = index;
    }
  }

  function clear() {
    Object.values(terminalInstances.value).forEach((term) => {
      term.dispose();
    });
    terminalInstances.value = {};
    terminals.value = [];
    activeIndex.value = 0;
  }

  return {
    terminalInstances,
    terminals,
    activeIndex,
    getActiveTerminal,
    getActiveIndex,
    getActiveTerminalInstance,
    getActiveTerminalTab,
    addTerminal,
    removeTerminal,
    setActiveIndex,
    clear,
  };
});
