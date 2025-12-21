# 项目架构快速参考

生成时间：2025-12-21
扫描范围：frontend/src/components/chat

## 项目类型
- 类型：全栈
- 应用类型：桌面应用（Tauri）

## 技术栈
- 主要语言：TypeScript (5.9.3)
- 核心框架：Vue 3.5.24 (Composition API)
- UI框架：Element Plus 2.8.0
- 状态管理：Pinia 3.0.0
- 构建工具：Vite 7.3.0
- 包管理器：pnpm 10.20.0
- 桌面框架：Tauri 2.0.0
- 样式：Tailwind CSS 4.1.18

## 目录结构
```
frontend/
├── src/
│   ├── components/
│   │   ├── chat/           # 聊天相关组件
│   │   ├── editor/         # 代码编辑器
│   │   ├── file-explorer/  # 文件浏览器
│   │   ├── layout/         # 布局组件
│   │   ├── terminal/       # 终端组件
│   │   └── settings/       # 设置组件
│   ├── stores/             # Pinia状态管理
│   ├── services/           # 服务层（Tauri命令）
│   ├── utils/              # 工具函数
│   └── pages/              # 页面组件
```

## 代码组织
- 命名规范：camelCase (变量/函数), PascalCase (组件/类型)
- 模块模式：ESM
- 分层结构：组件化架构（Component-based）
- 图标库：@element-plus/icons-vue

## 可复用组件
1. ChatPanel.vue：frontend/src/components/chat/ChatPanel.vue - 聊天面板，支持流式AI对话
2. ElDialog：Element Plus对话框组件 - 用于弹窗展示
3. ElIcon：Element Plus图标组件 - 统一图标展示
4. ElButton：Element Plus按钮组件 - 交互按钮
5. useAppStore：frontend/src/stores/appStore.ts - 应用全局状态
6. useFileStore：frontend/src/stores/filesStore.ts - 文件状态管理

## 代码风格
- 配置文件：.prettierrc (已配置)
- 缩进：2个空格
- 引号：单引号
- 脚本：使用 `<script setup lang="ts">` 语法
- 其他约定：
  - TypeScript严格模式
  - Vue 3 Composition API
  - 响应式变量使用 ref/computed
  - 生命周期钩子：onMounted/onBeforeUnmount

## 测试
- 测试框架：未检测到测试框架
- 测试位置：N/A
- 覆盖率要求：N/A

## 建议实施位置
基于功能类型的建议目录：
- UI组件：frontend/src/components/[feature]
- 页面组件：frontend/src/pages/[feature]
- 状态管理：frontend/src/stores/[feature]Store.ts
- 类型定义：frontend/src/utils/types.ts
- 工具函数：frontend/src/utils/[feature].ts
- Tauri命令：frontend/src/services/tauri/commands.ts

## 当前文件分析（ChatPanel.vue）
- 位置：frontend/src/components/chat/ChatPanel.vue
- 当前功能：
  - AI聊天对话（流式响应）
  - 关联文件管理（ElDialog弹窗）
  - 消息历史展示
  - 模型选择
  - Code CLI选择
- 使用的Element Plus组件：
  - ElDialog（关联文件弹窗）
  - ElButton、ElInput、ElSelect
  - ElIcon、ElTag、ElTooltip、ElMessageBox
- 现有图标：Link, Delete, Setting, Search, Folder, Document
- 现有状态：
  - messages（当前会话消息）
  - associatedFiles（关联文件）
  - showAssociateDialog（弹窗显示状态）
