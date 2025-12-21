# 路径处理统一指南

## 概述

本项目已实现统一的路径处理方式，所有路径都使用 `/`（正斜杠）作为分隔符，以确保在 Windows 和 Unix 系统间的兼容性。

## 前端路径工具 (pathUtils.ts)

### 核心函数

#### 1. `normalizePath(path: string): string`

**功能**: 将路径中的反斜杠 `\` 转换为正斜杠 `/`

**使用场景**: 当接收到来自操作系统或用户输入的路径时

```typescript
import { normalizePath } from '@/utils/pathUtils';

const windowsPath = 'C:\\Users\\project\\src';
const normalized = normalizePath(windowsPath); // 'C:/Users/project/src'
```

#### 2. `getDirectoryName(path: string): string`

**功能**: 从路径中提取目录名（最后一个路径段）

**使用场景**: 生成工作区名称或显示目录名

```typescript
import { getDirectoryName } from '@/utils/pathUtils';

const workspacePath = '/home/user/projects/my-project';
const dirName = getDirectoryName(workspacePath); // 'my-project'
```

#### 3. `getParentDirectory(path: string): string`

**功能**: 获取路径的父目录

**使用场景**: 文件操作后获取需要重载的目录

```typescript
import { getParentDirectory } from '@/utils/pathUtils';

const filePath = '/home/user/documents/file.txt';
const parent = getParentDirectory(filePath); // '/home/user/documents/'
```

#### 4. `joinPath(...segments: string[]): string`

**功能**: 安全地连接多个路径段，处理多余的斜杠

**使用场景**: 构建完整的文件或目录路径

```typescript
import { joinPath } from '@/utils/pathUtils';

const path = joinPath('/home/user', 'projects', 'file.txt');
// '/home/user/projects/file.txt'

// 也支持带反斜杠的输入
const mixed = joinPath('C:\\Users', 'project', 'src', 'app.ts');
// 'C:/Users/project/src/app.ts'
```

#### 5. `getFileName(path: string): string`

**功能**: 从路径中提取文件名

**使用场景**: 获取不包含目录的文件名

```typescript
import { getFileName } from '@/utils/pathUtils';

const filePath = '/home/user/documents/report.pdf';
const name = getFileName(filePath); // 'report.pdf'
```

#### 6. `getFileExtension(path: string): string`

**功能**: 从路径提取文件扩展名（不包含点）

**使用场景**: 判断文件类型

```typescript
import { getFileExtension } from '@/utils/pathUtils';

const filePath = '/home/user/document.pdf';
const ext = getFileExtension(filePath); // 'pdf'
```

#### 7. `isRootDirectory(path: string): boolean`

**功能**: 检查是否为根目录（单路径段）

**使用场景**: 判断路径是否为根级目录

```typescript
import { isRootDirectory } from '@/utils/pathUtils';

isRootDirectory('myproject'); // true
isRootDirectory('/home/user/projects'); // false
```

#### 8. `resolvePath(path: string): string`

**功能**: 解析相对路径（处理 `.` 和 `..`）

**使用场景**: 规范化包含相对路径的路径

```typescript
import { resolvePath } from '@/utils/pathUtils';

const path = '/home/user/../documents/./file.txt';
const resolved = resolvePath(path); // '/home/documents/file.txt'
```

## 后端路径规范化 (Rust)

### `normalize_path` 函数

位置: `src-tauri/src/utils/fs.rs`

**功能**: 将 Rust 的 `PathBuf` 转换为统一的 `/` 分隔符路径

**使用场景**: 所有返回给前端的路径都必须通过此函数处理

```rust
use crate::utils::fs::normalize_path;

let path_buf = entry.path();
let normalized = normalize_path(&path_buf.to_string_lossy().to_string());
// 返回给前端
```

## 迁移指南

### 对于已有代码

#### 之前的做法（需要避免）:

```typescript
// ❌ 不好的做法：分别处理 / 和 \
let name = path.split('/').pop() || '';
if (name === path) {
  name = path.split('\\').pop() || '';
}

// ❌ 字符串拼接：可能导致多余斜杠
const fullPath = `${directory}/${filename}`;
```

#### 推荐的做法：

```typescript
// ✅ 好的做法：使用专用工具函数
import { getDirectoryName, joinPath } from '@/utils/pathUtils';

const name = getDirectoryName(path);
const fullPath = joinPath(directory, filename);
```

## 存储中使用 Stores 的路径处理

### filesStore.ts

- 所有路径都通过 `normalizePath()` 标准化
- 路径拼接使用 `joinPath()`
- 路径比较前先规范化

### appStore.ts

- 工作区路径在创建时规范化
- 路径创建使用统一的工具函数

## 最佳实践

1. **总是规范化输入路径**

   ```typescript
   const userPath = // 来自用户输入或系统
   const normalized = normalizePath(userPath);
   ```

2. **使用 `joinPath` 而不是字符串拼接**

   ```typescript
   // ✅ 好
   const filePath = joinPath(dir, subdir, filename);

   // ❌ 避免
   const filePath = `${dir}/${subdir}/${filename}`;
   ```

3. **前后端保持一致**
   - 前端: 使用 `pathUtils` 工具函数
   - 后端: 使用 `normalize_path` 函数
   - 两端都确保返回 `/` 分隔符的路径

4. **路径比较时总是规范化**

   ```typescript
   // ✅ 好
   if (normalizePath(path1) === normalizePath(path2)) {
     // 处理相等情况
   }
   ```

5. **显示路径给用户**
   - 在 Windows 上可以考虑转换为 `\` 以符合用户期望
   - 但内部逻辑总是使用 `/`

## 常见问题

### Q: 为什么要统一使用 `/`？

**A**: 因为 JavaScript/TypeScript 原生支持 `/` 作为路径分隔符，无论在什么平台上。这避免了跨平台的复杂性。

### Q: 我应该在哪里调用 `normalizePath`？

**A**:

- 接收来自后端的路径：后端已规范化，但在前端处理前再规范化一次更安全
- 接收用户输入：总是规范化
- 构建路径：使用 `joinPath` 自动处理

### Q: 旧代码中的 `split('/')` 还能用吗？

**A**: 可以，但应该改用专门的工具函数。例如：

```typescript
// 旧: path.split('/')[0]
// 新: getDirectoryName(path)
```
