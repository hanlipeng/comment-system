# 留言及抽奖管理系统

本项目是一个基于 **Tauri v2**、**Vue 3** 和 **Rust** 构建的跨平台留言及抽奖管理系统。它包含一个桌面管理端应用和一个针对移动端优化的 Web 客户端。

## 项目概览

- **后端**: 使用 Rust 编写。Tauri 处理桌面端命令，Axum 处理 Web 端 API，SQLx/SQLite 负责数据持久化。
- **桌面管理端**: 位于根目录 `src`，使用 Vue 3 + TypeScript 构建。
- **Web 移动端**: 位于 `web-client/`，使用 Vue 3 + TypeScript + Tailwind CSS 构建，专为移动设备优化。
- **通信机制**: 管理端应用通过 Tauri Commands 与后端交互；Web 客户端通过 REST API (Axum) 与后端通信。

## 功能特点

### 留言管理
- 用户可以通过 Web 客户端提交留言
- 管理员可在桌面端查看、审核和管理所有留言
- 支持留言状态的跟踪（待审核、已通过、已拒绝）

### 抽奖系统
- 支持创建和配置抽奖活动
- 从已通过审核的留言中随机抽取获奖者
- 支持设置抽奖数量和中奖概率
- 抽奖结果实时公布

### 跨平台支持
- **桌面管理端**: 提供完整的admin管理界面，适合管理员日常操作
- **Web 移动端**: 响应式设计，适配各种移动设备，方便用户参与留言和查看中奖结果

## 技术栈

- **框架**: [Tauri v2](https://v2.tauri.app/), [Vue 3](https://vuejs.org/)
- **语言**: [Rust](https://www.rust-lang.org/), [TypeScript](https://www.typescriptlang.org/)
- **后端库**: Axum (HTTP 服务), SQLx (数据库), Tokio (异步运行时)
- **前端库**: Vite, Tailwind CSS (Web 客户端)
- **数据库**: SQLite

## 目录结构

- `src-tauri/`: Rust 后端代码
  - `src/api/`: Tauri 命令处理器
  - `src/core/`: 业务逻辑、领域模型和 Port 接口定义（六边形架构）
  - `src/storage/`: 数据库实现 (Adapters)
  - `src/server/`: Axum HTTP 服务器实现
  - `tests/`: 集成测试和逻辑测试
- `src/`: 桌面管理端前端代码
  - `components/`: Vue 组件
  - `api/`: 对 Tauri 命令的 TypeScript 封装
- `web-client/`: 移动端 Web 客户端代码
  - `src/api/client.ts`: REST API 客户端及 Mock 数据

## 快速开始

### 环境要求

- Rust 工具链 (cargo, rustc)
- Node.js 和 npm
- [Tauri CLI](https://v2.tauri.app/reference/cli/): `cargo install tauri-cli`

### 安装依赖

在项目根目录下运行：

```bash
make install
```

这会同时安装 Tauri 应用和 Web 客户端的所有依赖。

### 开发模式

启动 Tauri 开发模式（同时启动桌面窗口和 Web 端构建）：

```bash
make dev
```

如果只想单独开发 Web 客户端（使用 Mock 数据）：

```bash
make web-dev
```

### 构建发布

构建生产环境的 Linux 软件包：

```bash
make build
```

交叉编译 Windows 版本（需要安装 `x86_64-pc-windows-gnu` 目标）：

```bash
make build-windows
```

## 测试

运行所有 Rust 测试：

```bash
cargo test
```

运行特定的逻辑测试：

```bash
cargo test --test logic_tests
```

## 开发规范

- **架构**: 严格遵循 **Port/Adapter (六边形架构)**。
- **语言**: 代码实体使用英文命名，UI 界面使用简体中文。
- **前端**: 始终使用 `<script setup lang="ts">` 模式。
- **数据库**: 所有 ID 使用 UUID v4，时间戳存储为 i64 毫秒数。
