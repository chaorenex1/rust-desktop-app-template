/**
 * File icon utilities using vscode-material-icon-theme
 * Maps file extensions and names to Material Design icons
 */

import { getIconForFilePath, getIconForDirectoryPath } from 'vscode-material-icons';

export interface FileIconConfig {
  icon: string;
  color: string;
  isImage?: boolean; // Whether this is an image URL
}

// Base URL for Material Design icons (served from public directory)
const ICONS_BASE_URL = 'src/assets/icons';

// Default icons (fallback)
const DEFAULT_FILE_ICON = 'file.svg';
const DEFAULT_FOLDER_ICON = 'folder.svg';
const OPEN_FOLDER_ICON = 'folder-open.svg';

/**
 * Get icon URL from CDN
 */
function getIconUrl(iconName: string): string {
  console.debug(`Getting icon URL for ${iconName}`);
  return `${ICONS_BASE_URL}/${iconName}.svg`;
}

/**
 * Icon color mapping based on file type
 * These colors complement the Material Design icons
 */
const ICON_COLORS: Record<string, string> = {
  // Languages
  javascript: '#f7df1e',
  typescript: '#3178c6',
  vue: '#42b883',
  react: '#61dafb',
  python: '#3776ab',
  rust: '#ce422b',
  go: '#00add8',
  java: '#b07219',
  
  // Styles
  css: '#1572b6',
  scss: '#c6538c',
  sass: '#c6538c',
  less: '#1d365d',
  
  // Markup
  html: '#e34c26',
  xml: '#ff6600',
  
  // Data
  json: '#ffd700',
  yaml: '#cb171e',
  
  // Default
  default: '#90a4ae',
};

/**
 * Get file icon configuration based on filename
 */
export function getFileIcon(filename: string): FileIconConfig {
  try {
    // Get icon name from vscode-material-icons
    const iconName = getIconForFilePath(filename);
    
    // Determine color based on file extension or type
    const extension = filename.split('.').pop()?.toLowerCase() || '';
    let color = ICON_COLORS.default;
    
    // Map extensions to colors
    if (iconName.includes('javascript') || iconName.includes('js')) {
      color = ICON_COLORS.javascript;
    } else if (iconName.includes('typescript') || iconName.includes('ts')) {
      color = ICON_COLORS.typescript;
    } else if (iconName.includes('vue')) {
      color = ICON_COLORS.vue;
    } else if (iconName.includes('react')) {
      color = ICON_COLORS.react;
    } else if (iconName.includes('python')) {
      color = ICON_COLORS.python;
    } else if (iconName.includes('rust')) {
      color = ICON_COLORS.rust;
    } else if (iconName.includes('go')) {
      color = ICON_COLORS.go;
    } else if (iconName.includes('java')) {
      color = ICON_COLORS.java;
    } else if (iconName.includes('css') || iconName.includes('scss') || iconName.includes('sass')) {
      color = extension === 'css' ? ICON_COLORS.css : ICON_COLORS.scss;
    } else if (iconName.includes('html')) {
      color = ICON_COLORS.html;
    } else if (iconName.includes('json')) {
      color = ICON_COLORS.json;
    } else if (iconName.includes('yaml') || iconName.includes('yml')) {
      color = ICON_COLORS.yaml;
    }
    
    return {
      icon: getIconUrl(iconName),
      color,
      isImage: true,
    };
  } catch (error) {
    // Fallback to default icon
    return {
      icon: getIconUrl(DEFAULT_FILE_ICON),
      color: ICON_COLORS.default,
      isImage: true,
    };
  }
}

/**
 * Get folder icon
 */
export function getFolderIcon(isOpen = false, folderName?: string): FileIconConfig {
  try {
    let iconName: string;
    
    // vscode-material-icons doesn't distinguish open/closed in getIconForDirectoryPath
    // So we use the directory name to get the appropriate icon
    iconName = folderName ? getIconForDirectoryPath(folderName) : (isOpen ? OPEN_FOLDER_ICON : DEFAULT_FOLDER_ICON);
    
    return {
      icon: getIconUrl(iconName),
      color: '#90a4ae',
      isImage: true,
    };
  } catch (error) {
    return {
      icon: getIconUrl(isOpen ? OPEN_FOLDER_ICON : DEFAULT_FOLDER_ICON),
      color: '#90a4ae',
      isImage: true,
    };
  }
}

/**
 * Get icon for a file or directory
 */
export function getIcon(filename: string, isDirectory: boolean, isOpen = false): FileIconConfig {
  if (isDirectory) {
    return getFolderIcon(isOpen, filename);
  }
  return getFileIcon(filename);
}
