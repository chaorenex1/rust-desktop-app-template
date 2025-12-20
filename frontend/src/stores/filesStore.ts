import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import type { FileItem, FileContent } from '@/utils/types';
import { listFiles, readFile, writeFile, createDirectory } from '@/services/tauri/commands';
import { getFileIcon } from '@/utils/helpers';

export const useFileStore = defineStore('files', () => {
  // State
  const currentDirectory = ref<string>('.');
  const rootDirectory = ref<string>('.');
  const files = ref<FileItem[]>([]);
  const openedFiles = ref<FileContent[]>([]);
  const currentFile = ref<FileItem | null>(null);
  const activeFileIndex = ref<number>(-1);
  const isLoading = ref(false);
  const error = ref<string | null>(null);

  // 目录树数据结构：存储每个目录的下级目录和文件
  // key: 目录路径, value: 该目录下的文件和子目录列表
  const directoryCache = ref<Map<string, FileItem[]>>(new Map());

  // Getters
  const activeFile = computed(() => {
    if (activeFileIndex.value >= 0 && activeFileIndex.value < openedFiles.value.length) {
      return openedFiles.value[activeFileIndex.value];
    }
    return null;
  });

  const hasUnsavedChanges = computed(() => {
    return openedFiles.value.some((file) => file.modified);
  });

  // 获取指定目录的子项（从缓存中）
  const getDirectoryChildren = computed(() => {
    return (path: string): FileItem[] | undefined => {
      return directoryCache.value.get(path);
    };
  });

  // 检查目录是否已经加载过
  const isDirectoryLoaded = computed(() => {
    return (path: string): boolean => {
      return directoryCache.value.has(path);
    };
  });

  // Actions
  async function loadDirectory(path?: string) {
    try {
      isLoading.value = true;
      error.value = null;

      const targetPath = path || currentDirectory.value;

      // 如果是新的根目录，清空已打开的文件
      if (path && path !== currentDirectory.value) {
        openedFiles.value = [];
        activeFileIndex.value = -1;
      }

      const fileList = await listFiles(targetPath);
      files.value = fileList;

      // 将加载的目录数据存入缓存
      directoryCache.value.set(targetPath, fileList);

      currentDirectory.value = targetPath;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '加载目录失败';
      throw err;
    } finally {
      isLoading.value = false;
    }
  }

  async function openFile(path: string) {
    try {
      error.value = null;
      // Check if file is already opened
      const existingIndex = openedFiles.value.findIndex((file) => file.path === path);
      if (existingIndex >= 0) {
        activeFileIndex.value = existingIndex;
        return openedFiles.value[existingIndex];
      }
      const fileItem = files.value.find((file) => file.path === path);
      if (!fileItem) {
        error.value = `文件未找到: ${path}`;
        throw new Error(error.value);
      }

      currentFile.value = fileItem;

      const fileContent = await readFile(path);

      fileContent.language = getFileIcon(fileItem);
      fileContent.size = fileItem.size || 0;

      openedFiles.value.push(fileContent);
      activeFileIndex.value = openedFiles.value.length - 1;
      return fileContent;
    } catch (err) {
      error.value = err instanceof Error ? err.message : '打开文件失败';
      throw err;
    }
  }

  async function saveFile(content?: string) {
    try {
      error.value = null;

      const file = activeFile.value;
      if (!file) {
        throw new Error('没有活动的文件');
      }

      const saveContent = content || file.content;
      await invoke('write_file', {
        path: file.path,
        content: saveContent,
      });

      // Update file content and mark as saved
      if (activeFileIndex.value >= 0) {
        openedFiles.value[activeFileIndex.value] = {
          ...file,
          content: saveContent,
          modified: false,
        };
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '保存文件失败';
      throw err;
    }
  }

  async function saveAllFiles() {
    try {
      error.value = null;

      for (const file of openedFiles.value) {
        if (file.modified) {
          await invoke('write_file', {
            path: file.path,
            content: file.content,
          });
          file.modified = false;
        }
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '保存所有文件失败';
      throw err;
    }
  }

  async function createFile(name: string, isDirectory = false) {
    try {
      error.value = null;

      const path = `${currentDirectory.value}/${name}`;

      if (isDirectory) {
        await invoke('create_directory', { path });
      } else {
        await invoke('create_file', { path });
      }

      // Reload directory
      await loadDirectory(currentDirectory.value);

      // 删除相关目录缓存
      removeDirectoryCache(currentDirectory.value);

      // If it's a file, open it
      if (!isDirectory) {
        await openFile(path);
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '创建文件失败';
      throw err;
    }
  }

  async function deleteFile(path: string) {
    try {
      error.value = null;

      // Check if file is opened
      const fileIndex = openedFiles.value.findIndex((file) => file.path === path);
      if (fileIndex >= 0) {
        // Close file if it's opened
        closeFile(path);
      }

      // Delete file
      await invoke('delete_file', { path });

      // Reload directory
      await loadDirectory(currentDirectory.value);

      // 删除相关目录缓存
      removeDirectoryCache(currentDirectory.value);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除文件失败';
      throw err;
    }
  }

  async function renameFile(oldPath: string, newName: string) {
    try {
      error.value = null;

      const newPath = `${currentDirectory.value}/${newName}`;
      await invoke('rename_file', { oldPath, newPath });

      // Update opened files
      const fileIndex = openedFiles.value.findIndex((file) => file.path === oldPath);
      if (fileIndex >= 0) {
        const existing = openedFiles.value[fileIndex]!;
        openedFiles.value[fileIndex] = {
          path: newPath,
          content: existing.content,
          language: existing.language,
          modified: existing.modified,
        };
      }

      // Reload directory
      await loadDirectory(currentDirectory.value);

      // 删除相关目录缓存
      removeDirectoryCache(currentDirectory.value);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '重命名文件失败';
      throw err;
    }
  }

  // 从磁盘刷新当前活动文件内容（不标记为已修改）
  function refreshActiveFileContentFromDisk(content: string) {
    if (activeFileIndex.value >= 0) {
      const existing = openedFiles.value[activeFileIndex.value]!;
      openedFiles.value[activeFileIndex.value] = {
        path: existing.path,
        content,
        language: existing.language,
        modified: false,
      };
    }
  }

  function closeFile(path: string) {
    const fileIndex = openedFiles.value.findIndex((file) => file.path === path);
    if (fileIndex >= 0) {
      openedFiles.value.splice(fileIndex, 1);

      // 如果关闭的是当前文件之前的文件，索引需要减1
      if (activeFileIndex.value > fileIndex) {
        activeFileIndex.value--;
      }
      // 如果关闭的是当前文件
      else if (activeFileIndex.value === fileIndex) {
        // 如果还有文件，保持当前索引或移到前一个
        if (openedFiles.value.length > 0) {
          activeFileIndex.value = Math.min(fileIndex, openedFiles.value.length - 1);
        } else {
          activeFileIndex.value = -1;
        }
      }
    }
  }

  function closeAllFiles() {
    openedFiles.value = [];
    activeFileIndex.value = -1;
  }

  function setActiveFile(index: number) {
    if (index >= 0 && index < openedFiles.value.length) {
      activeFileIndex.value = index;
    }
  }

  function clearError() {
    error.value = null;
  }

  function getRootDirectory() {
    return rootDirectory.value;
  }

  function setRootDirectory(path: string) {
    rootDirectory.value = path;
  }

  // 加载子目录（用于懒加载）
  async function loadSubDirectory(path: string): Promise<FileItem[]> {
    try {
      // 如果已经缓存，直接返回
      if (directoryCache.value.has(path)) {
        return directoryCache.value.get(path)!;
      }

      // 否则加载并缓存
      const fileList = await listFiles(path);
      directoryCache.value.set(path, fileList);
      files.value.push(...fileList);
      return fileList;
    } catch (err) {
      console.error(`Failed to load subdirectory ${path}:`, err);
      throw err;
    }
  }

  // 清空目录缓存
  function clearDirectoryCache() {
    directoryCache.value.clear();
  }

  // 删除指定目录的缓存
  function removeDirectoryCache(path: string) {
    directoryCache.value.delete(path);
  }

  return {
    // State
    currentDirectory,
    files,
    openedFiles,
    currentFile,
    activeFileIndex,
    isLoading,
    error,
    directoryCache,

    // Getters
    activeFile,
    hasUnsavedChanges,
    getDirectoryChildren,
    isDirectoryLoaded,

    // Actions
    loadDirectory,
    loadSubDirectory,
    openFile,
    saveFile,
    saveAllFiles,
    createFile,
    deleteFile,
    renameFile,
    refreshActiveFileContentFromDisk,
    closeFile,
    closeAllFiles,
    setActiveFile,
    clearError,
    getRootDirectory,
    setRootDirectory,
    clearDirectoryCache,
    removeDirectoryCache,
  };
});
