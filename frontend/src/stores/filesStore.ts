import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import type { FileItem, FileContent } from '@/utils/types';
import { normalizePath, getParentDirectory, joinPath } from '@/utils/pathUtils';
import {
  listFiles,
  readFile,
  writeFile,
  createDirectory,
  deleteDirectory as deleteDirectoryCommand,
  deleteFile as deleteFileCommand,
  renameFile as renameFileCommand,
} from '@/services/tauri/commands';
import { getFileIcon } from '@/utils/helpers';

export const useFileStore = defineStore('files', () => {
  // State
  const currentDirectory = ref<string>('.');
  const rootDirectory = ref<string>('.');
  const files = ref<FileItem[]>([]);
  const openedFiles = ref<FileContent[]>([]);
  const activeFileIndex = ref<number>(-1);
  const currentFile = ref<FileItem | null>(null);
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

  const getRootDirectory = computed(() => {
    return rootDirectory.value;
  });

  // Get the current directory
  const getCurrentDirectory = computed(() => {
    return currentDirectory.value;
  });

  const getCurrentFile = computed(() => {
    return currentFile.value;
  });

  // const activeFileIndex = computed(() => {
  //   return openedFiles.value.findIndex((file) => file.path === currentFile.value?.path);
  // });

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
  async function loadDirectory(path?: string): Promise<FileItem[]> {
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
      if (files.value.length === 0) {
        files.value = fileList;
      } else {
        files.value = [...files.value, ...fileList];
      }

      // 将加载的目录数据存入缓存
      directoryCache.value.set(targetPath, fileList);

      currentDirectory.value = targetPath;
      return fileList;
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
      await writeFile(file.path, saveContent);
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
          await writeFile(file.path, file.content);
          file.modified = false;
        }
      }
    } catch (err) {
      error.value = err instanceof Error ? err.message : '保存所有文件失败';
      throw err;
    }
  }

  async function createFile(path: string, name: string, isDirectory = false) {
    try {
      error.value = null;
      const normalizedPath = normalizePath(path);
      const fullPath = joinPath(normalizedPath, name);
      console.debug('Creating new file:', { path, name, isDirectory, fullPath });
      if (isDirectory) {
        await createDirectory(fullPath);
      } else {
        await writeFile(fullPath, '');
      }

      // 删除相关目录缓存，强制重新加载
      removeDirectoryCache(path, false);

      // 重新加载当前目录
      await reloadDirectory(currentDirectory.value, false);
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
      await deleteFileCommand(path, false);

      // 删除相关目录缓存，强制重新加载
      removeDirectoryCache(currentDirectory.value);

      //Reload directory
      await reloadDirectory(currentDirectory.value, false);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除文件失败';
      throw err;
    }
  }

  async function deleteDirectory(path: string) {
    try {
      error.value = null;

      // Delete directory
      await deleteDirectoryCommand(path);

      // 删除相关目录缓存，强制重新加载
      removeDirectoryCache(path);
      // Reload directory
      await reloadDirectory(currentDirectory.value, true);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '删除目录失败';
      throw err;
    }
  }

  async function renameFile(oldPath: string, isDirectory: boolean, newName: string) {
    try {
      error.value = null;
      const normalizedCurrentDir = normalizePath(currentDirectory.value);
      const newPath = joinPath(normalizedCurrentDir, newName);
      await renameFileCommand(oldPath, newPath);

      // Update opened files
      const fileIndex = openedFiles.value.findIndex((file) => file.path === oldPath);
      if (fileIndex >= 0) {
        const existing = openedFiles.value[fileIndex]!;
        openedFiles.value[fileIndex] = {
          name: newName,
          path: newPath,
          content: existing.content,
          language: existing.language,
          modified: existing.modified,
          lineCount: existing.lineCount,
          size: existing.size,
        };
      }

      // 删除相关目录缓存，强制重新加载
      if (isDirectory) {
        removeDirectoryCache(oldPath);
      }
      // Reload directory
      await reloadDirectory(currentDirectory.value, false);
    } catch (err) {
      error.value = err instanceof Error ? err.message : '重命名文件失败';
      throw err;
    }
  }

  async function updateFileContent(content: string) {
    if (activeFileIndex.value >= 0) {
      const existing = openedFiles.value[activeFileIndex.value]!;
      openedFiles.value[activeFileIndex.value] = {
        name: existing.name,
        path: existing.path,
        content,
        language: existing.language,
        lineCount: existing.lineCount,
        size: existing.size,
        modified: true,
      };
    }
  }

  // 从磁盘刷新当前活动文件内容（不标记为已修改）
  function refreshActiveFileContentFromDisk(content: string) {
    if (activeFileIndex.value >= 0) {
      const existing = openedFiles.value[activeFileIndex.value]!;
      openedFiles.value[activeFileIndex.value] = {
        path: existing.path,
        name: existing.name,
        content,
        language: existing.language,
        modified: false,
        lineCount: existing.lineCount,
        size: existing.size,
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
    // 判断是否是当前文件
    if (currentFile.value?.path === path) {
      currentFile.value = null;
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
      currentDirectory.value = path;
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

  async function reloadDirectory(path: string, isDirectory: boolean = true) {
    console.debug('Reloading directory:', path, { isDirectory });
    const normalizedPath = normalizePath(path);
    if (isDirectory && normalizedPath === normalizePath(rootDirectory.value)) {
      await loadDirectory(normalizedPath);
    } else if (isDirectory) {
      await loadSubDirectory(normalizedPath);
    } else if (!isDirectory) {
      const subPath = getParentDirectory(normalizedPath);
      if (normalizePath(subPath) === normalizePath(rootDirectory.value)) {
        await loadDirectory(subPath);
      } else {
        await loadSubDirectory(subPath);
      }
    }
  }

  function removeDirectoryCache(path: string, isDirectory: boolean = true) {
    if (isDirectory) {
      directoryCache.value.delete(path);
      // 递归删除子目录缓存
      directoryCache.value.forEach((_, subPath) => {
        if (subPath.startsWith(path)) {
          directoryCache.value.delete(subPath);
        }
      });
    } else {
      const subPath = path.substring(0, path.lastIndexOf('/') + 1);
      directoryCache.value.delete(subPath);
      directoryCache.value.forEach((_, subPath) => {
        if (subPath.startsWith(subPath)) {
          directoryCache.value.delete(subPath);
        }
      });
      // 删除相关文件缓存
      files.value = files.value.filter((file) => !file.path.startsWith(path));
      openedFiles.value = openedFiles.value.filter((file) => !file.path.startsWith(path));
    }
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
    getCurrentDirectory,
    getCurrentFile,
    getRootDirectory,

    // Actions
    loadDirectory,
    loadSubDirectory,
    openFile,
    saveFile,
    saveAllFiles,
    createFile,
    deleteFile,
    deleteDirectory,
    renameFile,
    refreshActiveFileContentFromDisk,
    closeFile,
    closeAllFiles,
    setActiveFile,
    clearError,
    setRootDirectory,
    clearDirectoryCache,
    removeDirectoryCache,
    updateFileContent,
  };
});
