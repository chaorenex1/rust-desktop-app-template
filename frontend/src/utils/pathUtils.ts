/**
 * Path utility functions for unified path handling
 * All paths are normalized to use forward slash (/) as separator
 */

/**
 * Normalize path to use forward slash (/) as separator
 * Converts backslashes to forward slashes
 * @param path - The path to normalize
 * @returns Normalized path with forward slashes
 */
export function normalizePath(path: string): string {
  if (!path) return path;
  return path.replace(/\\/g, '/');
}

/**
 * Get the directory name from a path
 * @param path - The path to extract directory name from
 * @returns The directory name
 */
export function getDirectoryName(path: string): string {
  const normalized = normalizePath(path);
  const parts = normalized.split('/').filter(Boolean);
  return parts[parts.length - 1] || '';
}

/**
 * Get the parent directory path
 * @param path - The path to get parent from
 * @returns The parent directory path
 */
export function getParentDirectory(path: string): string {
  const normalized = normalizePath(path);
  const lastSlashIndex = normalized.lastIndexOf('/');
  if (lastSlashIndex === -1) {
    return normalized;
  }
  return normalized.substring(0, lastSlashIndex + 1);
}

/**
 * Join path segments with forward slash
 * @param segments - Path segments to join
 * @returns Joined path
 */
export function joinPath(...segments: string[]): string {
  return segments.map(normalizePath).filter(Boolean).join('/').replace(/\/+/g, '/'); // Replace multiple slashes with single slash
}

/**
 * Get file name with extension from path
 * @param path - The file path
 * @returns File name with extension
 */
export function getFileName(path: string): string {
  const normalized = normalizePath(path);
  const lastSlashIndex = normalized.lastIndexOf('/');
  if (lastSlashIndex === -1) {
    return normalized;
  }
  return normalized.substring(lastSlashIndex + 1);
}

/**
 * Get file extension from path
 * @param path - The file path
 * @returns File extension (without dot) or empty string
 */
export function getFileExtension(path: string): string {
  const fileName = getFileName(path);
  const dotIndex = fileName.lastIndexOf('.');
  if (dotIndex === -1 || dotIndex === 0) {
    return '';
  }
  return fileName.substring(dotIndex + 1).toLowerCase();
}

/**
 * Check if path is a root directory (single path segment)
 * @param path - The path to check
 * @returns true if path is a root directory
 */
export function isRootDirectory(path: string): boolean {
  const normalized = normalizePath(path);
  const parts = normalized.split('/').filter(Boolean);
  return parts.length === 1;
}

/**
 * Resolve relative path segments (. and ..)
 * @param path - The path to resolve
 * @returns Resolved path
 */
export function resolvePath(path: string): string {
  const normalized = normalizePath(path);
  const parts = normalized.split('/').filter(Boolean);
  const stack: string[] = [];

  for (const part of parts) {
    if (part === '.') {
      // Current directory - skip
      continue;
    } else if (part === '..') {
      // Parent directory - pop if possible
      if (stack.length > 0) {
        stack.pop();
      }
    } else {
      // Normal segment
      stack.push(part);
    }
  }

  return '/' + stack.join('/');
}
