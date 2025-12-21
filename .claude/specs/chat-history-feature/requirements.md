# 功能需求：聊天历史记录管理

生成时间：2025-12-21
清晰度评分：9/10
复杂度：中等偏复杂

## 核心功能
在ChatPanel中添加历史记录图标，点击后弹出对话框展示和管理会话历史，支持预览、加载、删除、重命名等操作。

## 用户故事
1. 作为用户，我想点击历史记录图标，以便在对话框中查看所有历史会话列表
2. 作为用户，我想点击某个历史会话进行预览，以便确认是否是我要找的对话
3. 作为用户，我想加载选中的历史会话到当前聊天面板，以便继续之前的对话
4. 作为用户，我想为会话添加备注或重命名，以便更好地识别和组织对话
5. 作为用户，我想删除不需要的历史会话，以便保持列表整洁
6. 作为用户，我想搜索历史会话，以便快速找到特定的对话
7. 作为用户，我希望会话自动保存，以便不丢失任何对话内容

## 验收标准
- [x] 在ChatPanel顶部工具栏添加历史记录图标按钮（靠近清空聊天按钮）
- [x] 点击图标弹出ElDialog对话框，展示历史会话列表
- [x] 历史列表显示：会话时间、消息数量、首条消息预览（或备注名称）
- [x] 支持搜索过滤功能（按内容或备注名称）
- [x] 点击会话在对话框中预览完整内容（右侧预览面板或展开详情）
- [x] 提供"加载会话"按钮，将选中会话加载到当前聊天面板
- [x] 支持重命名/添加备注功能
- [x] 支持删除单个历史会话（带确认提示）
- [x] 会话自动保存（发送消息后自动保存当前会话）
- [x] 使用Tauri后端持久化存储历史会话（JSON文件或SQLite）

## 技术约束
- 前端使用Element Plus的ElDialog组件
- 使用Vue 3 Composition API
- 需要实现Tauri后端命令：
  - `save_chat_session`: 保存会话
  - `load_chat_sessions`: 加载所有会话列表
  - `delete_chat_session`: 删除指定会话
  - `update_chat_session_name`: 更新会话名称/备注
- 存储格式：JSON文件（~/.aduib-app/chat-sessions/）或SQLite数据库
- 遵循现有代码风格和组件模式

## 影响范围
- 预计新建文件：3-4个
  - `src-tauri/src/chat_session.rs` (后端会话管理模块)
  - `frontend/src/components/chat/ChatHistoryDialog.vue` (历史记录对话框组件)
  - `frontend/src/stores/chatStore.ts` (可选，聊天状态管理)
- 预计修改文件：2-3个
  - `frontend/src/components/chat/ChatPanel.vue` (添加历史按钮和自动保存逻辑)
  - `frontend/src/services/tauri/commands.ts` (添加后端命令调用)
  - `frontend/src/utils/types.ts` (添加会话类型定义)
  - `src-tauri/src/lib.rs` (注册新的Tauri命令)

## 数据结构设计

### ChatSession（会话对象）
```typescript
interface ChatSession {
  id: string;                    // 唯一标识（UUID）
  name?: string;                 // 自定义名称/备注（可选）
  messages: ChatMessage[];       // 消息列表
  createdAt: string;             // 创建时间（ISO格式）
  updatedAt: string;             // 最后更新时间
  messageCount: number;          // 消息数量
  firstMessagePreview: string;   // 首条消息预览（前50字符）
}
```

### 后端存储格式
```rust
// chat-sessions/{session_id}.json
{
  "id": "uuid-v4",
  "name": "与AI讨论Vue架构",
  "messages": [...],
  "created_at": "2025-12-21T10:30:00Z",
  "updated_at": "2025-12-21T11:45:00Z"
}
```

## 非功能需求
- 性能要求：
  - 历史列表加载时间 < 500ms（假设100个会话）
  - 会话预览响应时间 < 200ms
  - 自动保存不阻塞用户操作（异步执行）
- 安全要求：
  - 会话文件仅存储在本地，不上传云端
  - 敏感信息（如API Key）不应保存在会话中
- 用户体验：
  - 自动保存失败时显示友好提示
  - 删除会话前需二次确认
  - 搜索支持实时过滤（输入时即时更新结果）
  - 列表按时间倒序排列（最新的在最上面）

## 实施优先级
1. **P0（核心功能）**：历史列表展示、保存/加载会话、Tauri后端存储
2. **P1（重要功能）**：删除会话、自动保存、搜索过滤
3. **P2（增强功能）**：重命名/备注、会话预览面板

## 风险点
1. **Tauri后端开发**：需要编写Rust代码，可能需要学习成本
2. **数据迁移**：如果用户已有聊天数据，需要考虑迁移方案
3. **并发保存**：多个会话同时自动保存时可能产生冲突
4. **存储空间**：长期使用可能积累大量历史数据，需考虑清理策略

## 未来扩展
- 导出会话为Markdown文件
- 会话分类/标签功能
- 会话搜索高级选项（按日期范围、模型类型等）
- 会话备份/恢复功能
