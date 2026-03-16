# 留言系统 Agent 指南 (AGENTS.md)

## 项目概览
本项目是一个基于 **Tauri v2**、**Vue 3** 和 **Rust** 构建的留言及抽奖管理系统。
- **后端**: Rust (Tauri 处理桌面端命令, Axum 处理 Web 端 API, SQLx/SQLite 负责存储)。
- **桌面端前端**: Vue 3 + TypeScript (位于根目录 `src`)。
- **Web 移动端**: Vue 3 + TypeScript + Tailwind CSS (位于 `web-client/`)。
- **通信**: 管理端应用使用 Tauri Commands；Web 客户端使用 REST API (Axum)。

## 目录结构
- `src-tauri/`: Rust 后端代码。
  - `src/api/`: Tauri 命令处理器。
  - `src/core/`: 业务逻辑、领域模型和 Port 接口定义。
  - `src/storage/`: 数据库实现 (Adapters)。
  - `src/server/`: Axum HTTP 服务器实现。
  - `tests/`: 集成测试和逻辑测试。
- `src/`: 桌面管理端前端。
  - `components/`: Vue 组件。
  - `api/`: 对 Tauri 命令的 TypeScript 封装。
- `web-client/`: 针对移动端优化的 Web 客户端。
  - `src/api/client.ts`: REST API 客户端及 Mock 数据。

## 常用命令

### 环境配置
- `make install`: 安装 Tauri 应用和 Web 客户端的所有依赖。
- `cargo install tauri-cli`: 开发 Tauri 必须安装的工具。

### 开发与构建
- `make dev`: 启动 Tauri 开发模式 (同时启动桌面窗口和 Web 端构建)。
- `make web-dev`: 在 `localhost:5173` 启动 Web 客户端开发服务器。
- `make build`: 构建生产环境的 Linux 软件包。
- `make build-windows`: 交叉编译 Windows 版本 (需要安装 `x86_64-pc-windows-gnu` 目标)。

### 测试与代码检查
- **Rust 测试**:
  - `cargo test`: 运行所有测试。
  - `cargo test --test logic_tests`: 运行核心逻辑测试。
  - `cargo test --test logic_tests test_draw_winner`: 运行特定的测试函数。
- **代码检查**:
  - `cargo clippy`: Rust Linter。
  - `npm run build` (在根目录或 web-client 目录): 运行类型检查 (`vue-tsc`)。

## 代码风格指南

### 通用规则
- **命名**: 所有代码实体（变量、函数、类等）使用 **英文** 命名。
- **UI 语言**: 界面文本和用户提示消息应使用 **简体中文**。
- **一致性**: 严格遵循现有的 **Port/Adapter (六边形架构)**。

### Rust (后端)
- **错误处理**: 
  - 业务逻辑和服务层使用 `anyhow::Result`。
  - 使用 `.context("...")` 为错误添加追溯信息。
  - Tauri 命令必须返回 `Result<T, String>`，并使用 `.map_err(|e| e.to_string())`。
- **异步**: 使用 `tokio` 和 `async_trait`。
- **命名**: 函数/变量使用 `snake_case`，结构体/枚举使用 `PascalCase`。
- **状态管理**: 通过 `tauri::State<'_, AppState>` 访问应用状态。
- **Port/Adapter 模式**:
  - 在 `core/mod.rs` 中定义 Trait (Ports)。
  - 在 `core/logic.rs` 中实现业务逻辑 (使用 Port Traits)。
  - 在 `storage/` 中实现数据库逻辑 (Adapters)。

### TypeScript/Vue (前端)
- **脚本模式**: 始终使用 `<script setup lang="ts">`。
- **命名**:
  - **组件**: `PascalCase` (如 `EventManager.vue`)。
  - **变量/函数**: `camelCase`。
  - **接口/类型**: `PascalCase`。
- **样式**:
  - **桌面管理端**: 标准 CSS，使用 `App.vue` 中定义的 CSS 变量。
  - **Web 客户端**: 使用 Tailwind CSS 工具类。
- **Mock 数据**: `web-client` 在开发模式下使用 Mock 模式 (检查 `import.meta.env.DEV`)。

### 数据库 (SQLite)
- **驱动**: 基于 `sqlx` 的 SQLite。
- **初始化**: 数据库 Schema 在 `src-tauri/src/storage/db.rs` 中管理。
- **主键**: 所有 ID 使用 `Uuid` (v4)，在 SQLite 中存储为字符串。
- **时间戳**: 存储为 `i64` (自 Epoch 以来的毫秒数)。

## 开发工作流

### 添加新功能 (从后端到前端)
1. **模型**: 在 `src-tauri/src/core/models.rs` 定义新模型或字段。
2. **Port**: 如有必要，更新 `src-tauri/src/core/mod.rs` 中的 Trait。
3. **逻辑**: 在 `src-tauri/src/core/logic.rs` 实现业务逻辑。
4. **Adapter**: 更新 `src-tauri/src/storage/sqlite.rs` 中的数据库代码。
5. **Command**: 在 `src-tauri/src/api/admin.rs` 添加 Tauri 命令并在 `lib.rs` 中注册。
6. **前端 API**: 在 `src/api/admin.ts` 添加对应的 TypeScript 调用。
7. **组件**: 在 Vue 组件中实现 UI。

### Web 客户端开发
- Web 客户端与运行在 Tauri 应用内部的 Axum 服务器通信。
- 开发期间，`web-client` 使用 Mock 数据。若要测试真实后端，必须运行 Tauri 应用 (`make dev`) 并确保应用内的服务器已启动。

### 测试说明
- `src-tauri/tests/` 中的集成测试使用真实的 SQLite 数据库 (通常是内存数据库或临时文件)。
- 修改核心逻辑或 API 处理器后，务必运行 `cargo test --test <suite>` 进行验证。

## 常用代码模式

### Rust: 易错的 Command 写法
```rust
#[tauri::command]
pub async fn my_command(state: State<'_, AppState>, param: String) -> Result<ReturnValue, String> {
    state.service.do_something(param)
        .await
        .map_err(|e| e.to_string())
}
```

### Rust: 错误上下文 (Error Context)
```rust
let data = self.storage.get_data(id)
    .await
    .context("从数据库获取数据失败")?;
```

### Vue: 带 Loading 状态的 API 调用
```typescript
const loading = ref(false);
const data = ref<MyData | null>(null);

async function fetchData() {
  loading.value = true;
  try {
    data.value = await adminApi.getData();
  } catch (e) {
    console.error("获取数据失败:", e);
  } finally {
    loading.value = false;
  }
}
```

### Vue: 响应式样式 (Web 客户端)
在 `web-client` 中使用 Tailwind 实现响应式设计：
```html
<div class="p-4 md:p-8 bg-white shadow-lg rounded-xl">
  <h1 class="text-2xl font-bold text-blue-600">{{ title }}</h1>
</div>
```

## 故障排查
- **构建失败**: 确保在根目录和 `web-client/` 目录都运行过 `npm install`。
- **数据库锁定**: 如果应用崩溃，请确保没有其他进程占用 SQLite 锁。
- **端口冲突**: Axum 服务器默认使用特定端口 (请检查 `settings.toml` 或 `config.rs`)。
