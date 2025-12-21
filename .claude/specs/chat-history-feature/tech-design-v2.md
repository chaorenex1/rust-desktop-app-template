# 技术方案（修订版）：聊天历史记录管理

生成时间：2025-12-21（修订）
基于PRD：requirements.md
修订原因：将数据存储从SQLite改为JSON文件存储

## 文件规划

### 新建文件

#### 1. `src-tauri/src/services/chat_session.rs`
- 职责：会话管理核心业务逻辑（文件系统操作）
- 类型：服务层模块
- 说明：实现会话的保存、加载、删除、更新等文件操作

#### 2. `src-tauri/src/tauri/chat_session_commands.rs`
- 职责：Tauri IPC命令定义
- 类型：命令处理器
- 说明：定义前端可调用的聊天会话相关命令

#### 3. `frontend/src/components/chat/ChatHistoryDialog.vue`
- 职责：历史记录对话框UI组件
- 类型：Vue组件
- 说明：展示历史会话列表，提供搜索、预览、加载、删除、重命名等功能

### 修改文件

#### 1. `frontend/src/components/chat/ChatPanel.vue`
- 改动类型：新增功能
- 改动说明：
  - 添加历史记录图标按钮（工具栏）
  - 实现会话自动保存逻辑（发送消息后）
  - 添加加载历史会话功能
  - 管理当前会话ID

#### 2. `frontend/src/services/tauri/commands.ts`
- 改动类型：新增命令
- 改动说明：添加4个新的Tauri命令调用函数：
  - `saveChatSession()` - 保存会话
  - `loadChatSessions()` - 加载会话列表
  - `deleteChatSession()` - 删除会话
  - `updateChatSessionName()` - 更新会话名称

#### 3. `frontend/src/utils/types/index.ts`
- 改动类型：新增类型
- 改动说明：添加`ChatSession`接口定义

#### 4. `src-tauri/src/main.rs`
- 改动类型：注册命令
- 改动说明：在`invoke_handler`中注册4个新命令

#### 5. `src-tauri/src/services/mod.rs`
- 改动类型：导出模块
- 改动说明：添加`pub mod chat_session;`

#### 6. `src-tauri/src/tauri/mod.rs`
- 改动类型：导出模块
- 改动说明：添加`pub mod chat_session_commands;`

## 核心接口定义

### 前端类型定义（TypeScript）

#### ChatSession 接口
```typescript
export interface ChatSession {
  id: string;                    // UUID
  name?: string;                 // 自定义名称/备注（可选）
  messages: ChatMessage[];       // 消息列表
  createdAt: string;             // ISO 8601格式
  updatedAt: string;             // ISO 8601格式
  messageCount: number;          // 消息数量
  firstMessagePreview: string;   // 首条消息预览（前100字符）
}
```

### 后端数据模型（Rust）

#### ChatSession 结构体
```rust
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatSession {
    pub id: String,
    pub name: Option<String>,
    pub messages: Vec<ChatMessage>,
    pub created_at: String,      // ISO 8601
    pub updated_at: String,      // ISO 8601
    pub message_count: usize,
    pub first_message_preview: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub role: String,           // "user" | "assistant" | "system"
    pub content: String,
    pub timestamp: String,
    pub files: Option<Vec<String>>,
    pub model: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveChatSessionRequest {
    pub session_id: Option<String>,  // 如果为None则新建，否则更新
    pub name: Option<String>,
    pub messages: Vec<ChatMessage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateChatSessionNameRequest {
    pub session_id: String,
    pub name: String,
}
```

### 文件存储结构

#### 存储目录
```
~/.aduib-app/
└── chat-sessions/
    ├── {session_id_1}.json
    ├── {session_id_2}.json
    ├── {session_id_3}.json
    └── ...
```

#### 单个会话文件格式（{session_id}.json）
```json
{
  "id": "550e8400-e29b-41d4-a716-446655440000",
  "name": "讨论Vue 3架构",
  "messages": [
    {
      "id": "msg-1",
      "role": "user",
      "content": "你好，我想了解Vue 3的响应式原理",
      "timestamp": "2025-12-21T10:30:00Z",
      "files": ["src/App.vue"],
      "model": "gpt-4"
    },
    {
      "id": "msg-2",
      "role": "assistant",
      "content": "Vue 3使用Proxy实现响应式...",
      "timestamp": "2025-12-21T10:30:15Z",
      "files": null,
      "model": "gpt-4"
    }
  ],
  "created_at": "2025-12-21T10:30:00Z",
  "updated_at": "2025-12-21T11:45:00Z",
  "message_count": 15,
  "first_message_preview": "你好，我想了解Vue 3的响应式原理"
}
```

### Tauri命令接口

#### 1. save_chat_session
```rust
#[tauri::command]
pub async fn save_chat_session(
    session_id: Option<String>,
    name: Option<String>,
    messages: Vec<ChatMessage>,
) -> Result<ChatSession, String>
```
- 输入：会话ID（可选）、会话名称（可选）、消息列表
- 输出：保存后的完整会话对象
- 说明：如果session_id为None则创建新会话，否则更新现有会话

#### 2. load_chat_sessions
```rust
#[tauri::command]
pub async fn load_chat_sessions(
    limit: Option<usize>,
) -> Result<Vec<ChatSession>, String>
```
- 输入：返回数量限制（可选，默认50）
- 输出：会话列表（按更新时间倒序）
- 说明：遍历所有JSON文件，解析并按时间排序

#### 3. delete_chat_session
```rust
#[tauri::command]
pub async fn delete_chat_session(
    session_id: String,
) -> Result<(), String>
```
- 输入：会话ID
- 输出：无（成功则返回Ok）
- 说明：删除指定的JSON文件

#### 4. update_chat_session_name
```rust
#[tauri::command]
pub async fn update_chat_session_name(
    session_id: String,
    name: String,
) -> Result<ChatSession, String>
```
- 输入：会话ID、新名称
- 输出：更新后的会话对象
- 说明：读取JSON文件，更新name字段，重新写入

## 数据流设计

### 保存会话流程
```
1. 用户发送消息 → ChatPanel.sendMessage()
2. 消息成功发送后 → 触发自动保存
3. 调用 saveChatSession(currentSessionId, messages)
4. 后端接收 → chat_session::save_session()
5. 生成/更新JSON文件 → 返回会话对象
6. 前端更新 currentSessionId（如果是新会话）
```

### 加载会话列表流程
```
1. 用户点击历史图标 → 打开 ChatHistoryDialog
2. 组件挂载 → 调用 loadChatSessions()
3. 后端遍历 ~/.aduib-app/chat-sessions/ 目录
4. 读取所有.json文件并解析
5. 按updated_at字段排序
6. 返回前N个会话（默认50个）
7. 前端渲染列表
```

### 加载单个会话流程
```
1. 用户点击某个历史会话 → 触发预览
2. 在对话框中展示该会话的所有消息
3. 用户点击"加载会话"按钮
4. 将该会话的messages赋值给ChatPanel.messages
5. 将该会话的id赋值给ChatPanel.currentSessionId
6. 关闭对话框，用户可继续对话
7. 继续发送消息时，更新该会话（非新建）
```

### 搜索过滤流程
```
1. 用户输入搜索关键词 → searchQuery
2. 前端过滤会话列表：
   - 匹配会话名称（name字段）
   - 匹配首条消息预览（firstMessagePreview字段）
3. 实时更新显示结果（computed属性）
```

## 集成点

### 与现有ChatPanel集成
- **工具栏按钮**：在清空聊天按钮旁边添加历史图标（使用`@element-plus/icons-vue`的`Clock`图标）
- **自动保存时机**：在`sendMessage()`函数成功发送后调用`autoSaveChatSession()`
- **会话ID管理**：添加`currentSessionId` ref变量，用于跟踪当前会话

### 与现有文件系统集成
- 使用现有的`utils/fs.rs`工具函数（如果有）
- 使用标准库的`std::fs`进行文件操作
- 确保目录存在（使用`std::fs::create_dir_all`）

### 与现有Tauri命令集成
- 在`main.rs`的`invoke_handler`中注册新命令
- 在`tauri/mod.rs`中导出新的命令模块
- 在`services/mod.rs`中导出新的服务模块

## 实施步骤

### 步骤1：实现服务层（文件操作）（预估：30分钟）
- 任务：
  - 创建`services/chat_session.rs`
  - 实现`save_session()` - 保存/更新会话到JSON文件
  - 实现`load_all_sessions()` - 读取所有会话文件
  - 实现`delete_session()` - 删除会话文件
  - 实现`update_session_name()` - 更新会话名称
  - 实现辅助函数：`get_sessions_dir()`, `ensure_sessions_dir_exists()`
- 产出：
  - `src-tauri/src/services/chat_session.rs`

### 步骤2：实现Tauri命令（预估：15分钟）
- 任务：
  - 创建`tauri/chat_session_commands.rs`
  - 定义4个命令函数，调用服务层
  - 在`main.rs`中注册4个命令
  - 在`mod.rs`中导出模块
- 产出：
  - `src-tauri/src/tauri/chat_session_commands.rs`
  - 修改 `src-tauri/src/main.rs`
  - 修改 `src-tauri/src/tauri/mod.rs`

### 步骤3：添加前端类型和命令调用（预估：10分钟）
- 任务：
  - 在`types/index.ts`添加`ChatSession`接口
  - 在`commands.ts`添加4个命令调用函数
- 产出：
  - 修改 `frontend/src/utils/types/index.ts`
  - 修改 `frontend/src/services/tauri/commands.ts`

### 步骤4：实现ChatHistoryDialog组件（预估：40分钟）
- 任务：
  - 创建组件骨架（ElDialog + 列表布局）
  - 实现会话列表展示（时间、消息数、预览）
  - 实现搜索过滤功能
  - 实现预览面板（点击展开显示完整消息）
  - 实现删除功能（带确认提示）
  - 实现重命名功能（inline编辑）
  - 实现加载会话功能（emit事件或回调）
- 产出：
  - `frontend/src/components/chat/ChatHistoryDialog.vue`

### 步骤5：修改ChatPanel集成历史功能（预估：20分钟）
- 任务：
  - 导入`Clock`图标和`ChatHistoryDialog`组件
  - 添加历史按钮到工具栏
  - 添加`showHistoryDialog` ref变量
  - 添加`currentSessionId` ref变量（用于跟踪当前会话）
  - 实现`autoSaveChatSession()`函数（在sendMessage成功后调用）
  - 实现`loadHistorySession(session)`函数（加载选中的会话）
  - 添加对话框组件到template
- 产出：
  - 修改 `frontend/src/components/chat/ChatPanel.vue`

### 步骤6：测试和调试（预估：20分钟）
- 任务：
  - 测试会话自动保存功能
  - 测试历史列表加载和展示
  - 测试搜索过滤功能
  - 测试删除会话功能
  - 测试重命名功能
  - 测试加载历史会话到当前面板
  - 检查文件系统权限问题
  - 修复发现的bug
- 产出：
  - 功能验证报告

**总预估时间：135分钟（约2小时15分钟）**

## 技术决策

### 决策1：使用JSON文件而非SQLite数据库
- 原因：
  - 用户明确要求使用文件存储
  - 实现更简单，不需要数据库迁移
  - 不需要Repository层，减少代码量
  - 每个会话独立文件，便于备份和迁移
  - 适合中小规模数据量（< 1000个会话）
- 权衡：
  - ❌ 搜索性能较差（需遍历所有文件）
  - ❌ 分页实现效率低
  - ✅ 实现简单，代码量少
  - ✅ 文件可读性强，便于调试

### 决策2：每个会话独立文件，而非单一JSON文件
- 原因：
  - 避免单个文件过大（随着会话增多可能达到MB级别）
  - 删除/更新单个会话更高效（无需重写整个文件）
  - 文件系统天然提供并发保护（不同会话可并发写入）
  - 备份和恢复更灵活（可选择性备份）
- 权衡：
  - ❌ 加载所有会话需要遍历目录
  - ✅ 单个操作性能更好

### 决策3：前端预览使用内存缓存，不请求单独接口
- 原因：
  - `load_chat_sessions`已返回完整消息列表
  - 减少网络请求，提升响应速度
  - 会话列表通常不会很大（限制50个），内存消耗可控

### 决策4：自动保存采用"upsert"模式
- 原因：
  - 首次保存创建新会话，后续保存更新现有会话
  - 用户无需手动创建会话，体验更流畅
  - 通过`currentSessionId`判断是新建还是更新

### 决策5：使用ElDialog嵌套而非路由跳转
- 原因：
  - 保持在聊天页面上下文，不中断用户流程
  - 与现有"关联文件"弹窗保持一致的交互模式
  - 实现简单，代码复用度高

## 服务层核心实现（伪代码）

### services/chat_session.rs

```rust
use std::fs;
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use chrono::Utc;
use uuid::Uuid;

// 获取会话存储目录
fn get_sessions_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("无法获取用户主目录")?;
    let sessions_dir = home.join(".aduib-app").join("chat-sessions");
    Ok(sessions_dir)
}

// 确保目录存在
fn ensure_sessions_dir_exists() -> Result<PathBuf, String> {
    let dir = get_sessions_dir()?;
    fs::create_dir_all(&dir).map_err(|e| format!("创建目录失败: {}", e))?;
    Ok(dir)
}

// 保存会话
pub fn save_session(
    session_id: Option<String>,
    name: Option<String>,
    messages: Vec<ChatMessage>,
) -> Result<ChatSession, String> {
    let dir = ensure_sessions_dir_exists()?;

    // 生成或使用现有ID
    let id = session_id.unwrap_or_else(|| Uuid::new_v4().to_string());
    let file_path = dir.join(format!("{}.json", id));

    // 构造会话对象
    let now = Utc::now().to_rfc3339();
    let first_message_preview = messages
        .first()
        .map(|m| {
            let content = &m.content;
            if content.len() > 100 {
                format!("{}...", &content[..100])
            } else {
                content.clone()
            }
        })
        .unwrap_or_default();

    let session = ChatSession {
        id: id.clone(),
        name,
        messages,
        created_at: if file_path.exists() {
            // 保留原创建时间
            let existing = load_session_by_id(&id)?;
            existing.created_at
        } else {
            now.clone()
        },
        updated_at: now,
        message_count: messages.len(),
        first_message_preview,
    };

    // 写入文件
    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("序列化失败: {}", e))?;
    fs::write(&file_path, json)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(session)
}

// 加载所有会话
pub fn load_all_sessions(limit: Option<usize>) -> Result<Vec<ChatSession>, String> {
    let dir = get_sessions_dir()?;

    if !dir.exists() {
        return Ok(Vec::new());
    }

    let entries = fs::read_dir(&dir)
        .map_err(|e| format!("读取目录失败: {}", e))?;

    let mut sessions = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|e| format!("读取目录项失败: {}", e))?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match serde_json::from_str::<ChatSession>(&content) {
                        Ok(session) => sessions.push(session),
                        Err(e) => eprintln!("解析会话文件失败 {:?}: {}", path, e),
                    }
                }
                Err(e) => eprintln!("读取会话文件失败 {:?}: {}", path, e),
            }
        }
    }

    // 按更新时间倒序排序
    sessions.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));

    // 限制返回数量
    if let Some(limit) = limit {
        sessions.truncate(limit);
    }

    Ok(sessions)
}

// 删除会话
pub fn delete_session(session_id: &str) -> Result<(), String> {
    let dir = get_sessions_dir()?;
    let file_path = dir.join(format!("{}.json", session_id));

    if !file_path.exists() {
        return Err(format!("会话不存在: {}", session_id));
    }

    fs::remove_file(&file_path)
        .map_err(|e| format!("删除文件失败: {}", e))?;

    Ok(())
}

// 更新会话名称
pub fn update_session_name(session_id: &str, name: String) -> Result<ChatSession, String> {
    let mut session = load_session_by_id(session_id)?;
    session.name = Some(name);
    session.updated_at = Utc::now().to_rfc3339();

    let dir = get_sessions_dir()?;
    let file_path = dir.join(format!("{}.json", session_id));

    let json = serde_json::to_string_pretty(&session)
        .map_err(|e| format!("序列化失败: {}", e))?;
    fs::write(&file_path, json)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    Ok(session)
}

// 辅助函数：根据ID加载单个会话
fn load_session_by_id(session_id: &str) -> Result<ChatSession, String> {
    let dir = get_sessions_dir()?;
    let file_path = dir.join(format!("{}.json", session_id));

    if !file_path.exists() {
        return Err(format!("会话不存在: {}", session_id));
    }

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("读取文件失败: {}", e))?;

    serde_json::from_str(&content)
        .map_err(|e| format!("解析JSON失败: {}", e))
}
```

## 风险点

### 风险1：大量历史会话导致性能问题
- 说明：遍历1000+个JSON文件会导致加载变慢
- 应对措施：
  - 限制默认加载数量（50个）
  - 实现分页加载（滚动加载更多）
  - 考虑缓存会话元数据（未来优化）

### 风险2：文件读写并发冲突
- 说明：快速连续保存同一会话可能导致文件损坏
- 应对措施：
  - 使用防抖（debounce）延迟保存操作
  - Rust的文件写入是原子性的（写到临时文件再重命名）
  - 添加文件锁保护（如需要）

### 风险3：文件系统权限问题
- 说明：用户可能没有~/.aduib-app目录的写权限
- 应对措施：
  - 检查目录创建和文件写入的权限
  - 提供友好的错误提示
  - 考虑降级到临时目录（作为备选）

### 风险4：JSON文件损坏
- 说明：应用崩溃或强制关闭可能导致JSON文件不完整
- 应对措施：
  - 使用`serde_json`的pretty print提高可读性
  - 解析失败时跳过该文件并记录错误日志
  - 定期备份会话文件（未来扩展）

## UI/UX 设计要点

### ChatPanel工具栏布局
```
[模型选择]  [Code CLI选择]  [历史记录图标🕒]  [关联文件📎]  [清空聊天🗑️]
```

### ChatHistoryDialog布局
```
┌─────────────────────────────────────────────────────────┐
│  聊天历史                                    [搜索框🔍]  │
├─────────────────────┬───────────────────────────────────┤
│ 会话列表 (左侧)     │ 预览面板 (右侧)                   │
│                     │                                   │
│ □ 讨论Vue架构       │ [会话名称: 讨论Vue架构]           │
│   10条消息          │                                   │
│   2025-12-20 14:30  │ User: 你好，我想了解Vue 3的...  │
│                     │                                   │
│ □ Python爬虫开发    │ Assistant: 很高兴帮助你...       │
│   25条消息          │                                   │
│   2025-12-19 09:15  │ User: 继续刚才的话题...          │
│                     │                                   │
│ [加载更多...]       │ [加载会话] [重命名] [删除]       │
└─────────────────────┴───────────────────────────────────┘
```

### 交互细节
1. **搜索实时过滤**：输入时立即更新列表
2. **点击会话显示预览**：右侧面板滚动显示所有消息
3. **双击会话直接加载**：快捷操作
4. **删除确认提示**：`ElMessageBox.confirm()`
5. **重命名inline编辑**：点击名称区域可编辑
6. **加载成功提示**：`ElMessage.success('已加载会话')`

## 性能优化建议

1. **限制加载数量**：默认加载50个会话
2. **搜索防抖**：搜索输入使用300ms防抖
3. **消息预览截取**：首条消息仅保存前100字符
4. **懒加载**：列表滚动到底部时加载更多
5. **会话元数据缓存**：考虑创建index.json缓存会话列表（未来优化）

## 安全考虑

1. **输入验证**：前端验证会话名称长度（≤100字符）
2. **路径验证**：确保session_id不包含路径分隔符（防止目录遍历攻击）
3. **XSS防护**：消息内容使用Vue的文本插值（自动转义）
4. **文件权限**：确保会话文件仅对当前用户可读写

## 未来扩展点

1. **会话导出**：导出为Markdown或JSON文件
2. **会话标签**：为会话添加标签分类
3. **会话备份**：定期备份到云端或其他目录
4. **全文搜索**：建立索引支持快速搜索
5. **会话统计**：分析对话趋势、常用模型等
6. **元数据缓存**：创建index.json避免每次遍历所有文件
