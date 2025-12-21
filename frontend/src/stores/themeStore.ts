import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { Theme, ColorScheme } from '@/utils/types';

export const useThemeStore = defineStore('theme', () => {
  // State
  const currentTheme = ref<Theme>('light');
  const colorScheme = ref<ColorScheme>('system');

  // Computed
  const isDarkMode = computed(() => {
    if (colorScheme.value === 'system') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches;
    }
    return colorScheme.value === 'dark';
  });

  const effectiveTheme = computed(() => {
    return colorScheme.value === 'system'
      ? isDarkMode.value
        ? 'dark'
        : 'light'
      : currentTheme.value;
  });

  // Actions
  const setTheme = (theme: Theme) => {
    currentTheme.value = theme;
    colorScheme.value = theme;
    applyTheme();
    saveToStorage();
  };

  const setColorScheme = (scheme: ColorScheme) => {
    colorScheme.value = scheme;
    if (scheme !== 'system') {
      currentTheme.value = scheme;
    }
    applyTheme();
    saveToStorage();
  };

  const toggleTheme = () => {
    const newTheme = currentTheme.value === 'light' ? 'dark' : 'light';
    setTheme(newTheme);
  };

  const applyTheme = () => {
    const themeToApply = effectiveTheme.value;

    console.log('Applying theme:', themeToApply);

    // Toggle dark class on html element
    document.documentElement.classList.toggle('dark', themeToApply === 'dark');

    // Load corresponding theme CSS file
    loadThemeCSS(themeToApply);
  };

  const loadThemeCSS = (theme: 'light' | 'dark') => {
    // Remove existing theme links
    const existingLinks = document.querySelectorAll('link[data-theme]');
    existingLinks.forEach((link) => link.remove());

    // Create and append new theme link
    const link = document.createElement('link');
    link.rel = 'stylesheet';
    link.href = `/src/styles/themes/${theme}.css`;
    link.setAttribute('data-theme', theme);

    // Add load event listener for debugging
    link.onload = () => {
      console.log(`Theme CSS loaded: ${theme}.css`);
    };
    link.onerror = () => {
      console.error(`Failed to load theme CSS: ${theme}.css`);
    };

    document.head.appendChild(link);
  };

  const saveToStorage = () => {
    localStorage.setItem('theme', currentTheme.value);
    localStorage.setItem('colorScheme', colorScheme.value);
  };

  // const loadFromStorage = () => {
  //   const savedTheme = localStorage.getItem('theme') as Theme | null;
  //   const savedColorScheme = localStorage.getItem('colorScheme') as ColorScheme | null;

  //   if (savedTheme) {
  //     currentTheme.value = savedTheme;
  //   }
  //   if (savedColorScheme) {
  //     colorScheme.value = savedColorScheme;
  //   }

  //   applyTheme();
  // };

  const resetToDefaults = () => {
    currentTheme.value = 'light';
    colorScheme.value = 'system';
    applyTheme();
    saveToStorage();
  };

  // Watch for system color scheme changes
  const mediaQuery = window.matchMedia('(prefers-color-scheme: dark)');
  const handleSystemThemeChange = () => {
    if (colorScheme.value === 'system') {
      applyTheme();
    }
  };

  // Initialize
  const initialize = (savedTheme: Theme, savedColorScheme: ColorScheme) => {
    console.log('Initializing theme with:', savedTheme, savedColorScheme);
    if (savedTheme) {
      currentTheme.value = savedTheme;
    }
    if (savedColorScheme) {
      colorScheme.value = savedColorScheme;
    }

    applyTheme();
    mediaQuery.addEventListener('change', handleSystemThemeChange);
  };

  // Cleanup
  const cleanup = () => {
    mediaQuery.removeEventListener('change', handleSystemThemeChange);
  };

  return {
    // State
    currentTheme,
    colorScheme,

    // Computed
    isDarkMode,
    effectiveTheme,

    // Actions
    setTheme,
    setColorScheme,
    toggleTheme,
    applyTheme,
    resetToDefaults,
    initialize,
    cleanup,
  };
});
