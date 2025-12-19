/**
 * File icon utilities inspired by vscode-material-icon-theme
 * Maps file extensions and names to icons and colors
 */

export interface FileIconConfig {
  icon: string;
  color: string;
}

// Default icons
const DEFAULT_FILE_ICON = '📄';
const DEFAULT_FOLDER_ICON = '📁';
const OPEN_FOLDER_ICON = '📂';

// File extension to icon mapping
const EXTENSION_ICONS: Record<string, FileIconConfig> = {
  // JavaScript/TypeScript
  js: { icon: '🟨', color: '#f7df1e' },
  jsx: { icon: '⚛️', color: '#61dafb' },
  ts: { icon: '🔷', color: '#3178c6' },
  tsx: { icon: '⚛️', color: '#61dafb' },
  mjs: { icon: '🟨', color: '#f7df1e' },
  cjs: { icon: '🟨', color: '#f7df1e' },

  // Vue/React/Angular
  vue: { icon: '💚', color: '#42b883' },
  svelte: { icon: '🧡', color: '#ff3e00' },

  // Styles
  css: { icon: '🎨', color: '#1572b6' },
  scss: { icon: '🎨', color: '#c6538c' },
  sass: { icon: '🎨', color: '#c6538c' },
  less: { icon: '🎨', color: '#1d365d' },

  // Markup
  html: { icon: '🌐', color: '#e34c26' },
  xml: { icon: '📋', color: '#ff6600' },
  svg: { icon: '🖼️', color: '#ffb13b' },

  // Data formats
  json: { icon: '📋', color: '#ffd700' },
  yaml: { icon: '📋', color: '#cb171e' },
  yml: { icon: '📋', color: '#cb171e' },
  toml: { icon: '📋', color: '#9c4221' },

  // Markdown
  md: { icon: '📝', color: '#083fa1' },
  mdx: { icon: '📝', color: '#083fa1' },

  // Programming languages
  rs: { icon: '🦀', color: '#ce422b' },
  go: { icon: '🐹', color: '#00add8' },
  py: { icon: '🐍', color: '#3776ab' },
  java: { icon: '☕', color: '#b07219' },
  c: { icon: '🔧', color: '#555555' },
  cpp: { icon: '🔧', color: '#f34b7d' },
  h: { icon: '🔧', color: '#a8b9cc' },
  hpp: { icon: '🔧', color: '#a8b9cc' },
  rb: { icon: '💎', color: '#cc342d' },
  php: { icon: '🐘', color: '#777bb4' },
  swift: { icon: '🍊', color: '#f05138' },
  kt: { icon: '🟣', color: '#7f52ff' },
  dart: { icon: '🎯', color: '#00b4ab' },

  // Shell scripts
  sh: { icon: '🐚', color: '#89e051' },
  bash: { icon: '🐚', color: '#89e051' },
  zsh: { icon: '🐚', color: '#89e051' },
  fish: { icon: '🐚', color: '#89e051' },
  ps1: { icon: '🔷', color: '#012456' },

  // Config files
  conf: { icon: '⚙️', color: '#6d6d6d' },
  config: { icon: '⚙️', color: '#6d6d6d' },
  ini: { icon: '⚙️', color: '#6d6d6d' },
  env: { icon: '🔑', color: '#eed202' },

  // Build/Package files
  lock: { icon: '🔒', color: '#6d6d6d' },

  // Images
  png: { icon: '🖼️', color: '#a074c4' },
  jpg: { icon: '🖼️', color: '#a074c4' },
  jpeg: { icon: '🖼️', color: '#a074c4' },
  gif: { icon: '🖼️', color: '#a074c4' },
  webp: { icon: '🖼️', color: '#a074c4' },
  ico: { icon: '🖼️', color: '#a074c4' },

  // Documents
  pdf: { icon: '📕', color: '#e32b2b' },
  doc: { icon: '📘', color: '#2b579a' },
  docx: { icon: '📘', color: '#2b579a' },
  xls: { icon: '📗', color: '#207245' },
  xlsx: { icon: '📗', color: '#207245' },

  // Archives
  zip: { icon: '🗜️', color: '#6d6d6d' },
  tar: { icon: '🗜️', color: '#6d6d6d' },
  gz: { icon: '🗜️', color: '#6d6d6d' },
  rar: { icon: '🗜️', color: '#6d6d6d' },
  '7z': { icon: '🗜️', color: '#6d6d6d' },

  // Git
  gitignore: { icon: '🚫', color: '#f54d27' },
  gitattributes: { icon: '🚫', color: '#f54d27' },

  // Database
  db: { icon: '🗄️', color: '#003b57' },
  sql: { icon: '🗄️', color: '#e38c00' },
  sqlite: { icon: '🗄️', color: '#003b57' },
};

// Filename to icon mapping (exact match)
const FILENAME_ICONS: Record<string, FileIconConfig> = {
  // Package managers
  'package.json': { icon: '📦', color: '#cb3837' },
  'package-lock.json': { icon: '🔒', color: '#cb3837' },
  'pnpm-lock.yaml': { icon: '🔒', color: '#f9ad00' },
  'yarn.lock': { icon: '🔒', color: '#2c8ebb' },
  'Cargo.toml': { icon: '📦', color: '#ce422b' },
  'Cargo.lock': { icon: '🔒', color: '#ce422b' },
  'go.mod': { icon: '📦', color: '#00add8' },
  'go.sum': { icon: '🔒', color: '#00add8' },

  // Config files
  'tsconfig.json': { icon: '🔷', color: '#3178c6' },
  'vite.config.ts': { icon: '⚡', color: '#646cff' },
  'vite.config.js': { icon: '⚡', color: '#646cff' },
  'tailwind.config.js': { icon: '🎨', color: '#38bdf8' },
  'tailwind.config.ts': { icon: '🎨', color: '#38bdf8' },
  'webpack.config.js': { icon: '📦', color: '#8dd6f9' },
  'rollup.config.js': { icon: '📦', color: '#ec4a3f' },
  '.prettierrc': { icon: '✨', color: '#f7b93e' },
  '.prettierrc.js': { icon: '✨', color: '#f7b93e' },
  '.prettierrc.cjs': { icon: '✨', color: '#f7b93e' },
  '.eslintrc': { icon: '🔍', color: '#4b32c3' },
  '.eslintrc.js': { icon: '🔍', color: '#4b32c3' },
  '.editorconfig': { icon: '⚙️', color: '#6d6d6d' },

  // Documentation
  'README.md': { icon: '📖', color: '#083fa1' },
  LICENSE: { icon: '⚖️', color: '#6d6d6d' },
  'CHANGELOG.md': { icon: '📋', color: '#083fa1' },

  // Git
  '.gitignore': { icon: '🚫', color: '#f54d27' },
  '.gitattributes': { icon: '🚫', color: '#f54d27' },

  // Docker
  Dockerfile: { icon: '🐳', color: '#2496ed' },
  'docker-compose.yml': { icon: '🐳', color: '#2496ed' },
  '.dockerignore': { icon: '🚫', color: '#2496ed' },

  // Environment
  '.env': { icon: '🔑', color: '#eed202' },
  '.env.local': { icon: '🔑', color: '#eed202' },
  '.env.development': { icon: '🔑', color: '#eed202' },
  '.env.production': { icon: '🔑', color: '#eed202' },
  '.env.example': { icon: '🔑', color: '#999999' },
};

/**
 * Get file icon configuration based on filename and extension
 */
export function getFileIcon(filename: string): FileIconConfig {
  // Check exact filename match first
  if (FILENAME_ICONS[filename]) {
    return FILENAME_ICONS[filename];
  }

  // Check for dotfiles with extensions (e.g., .prettierrc.js)
  const dotfileMatch = Object.keys(FILENAME_ICONS).find(
    (key) => filename.toLowerCase() === key.toLowerCase()
  );
  if (dotfileMatch) {
    return FILENAME_ICONS[dotfileMatch];
  }

  // Get extension
  const extension = filename.split('.').pop()?.toLowerCase() || '';

  if (EXTENSION_ICONS[extension]) {
    return EXTENSION_ICONS[extension];
  }

  // Default file icon
  return { icon: DEFAULT_FILE_ICON, color: '#6d6d6d' };
}

/**
 * Get folder icon
 */
export function getFolderIcon(isOpen = false): FileIconConfig {
  return {
    icon: isOpen ? OPEN_FOLDER_ICON : DEFAULT_FOLDER_ICON,
    color: '#90a4ae',
  };
}

/**
 * Get icon for a file or directory
 */
export function getIcon(filename: string, isDirectory: boolean, isOpen = false): FileIconConfig {
  if (isDirectory) {
    return getFolderIcon(isOpen);
  }
  return getFileIcon(filename);
}
