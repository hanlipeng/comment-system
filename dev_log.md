# 开发日志 (Dev Log)

## [2026-01-11] 初始化项目规划
- **任务**: 建立目录结构和初步 Roadmap。
- **进展**: 
    - [x] 创建 `docs/` 目录结构。
    - [x] 完成宏观需求文档 `docs/requirements/macro/req_001_initial.md`。
    - [x] 完成项目路线图 `docs/planning/roadmap.md`。

## [2026-01-11] 接口契约定义
- **任务**: 定义前后端交互规范。
- **进展**:
    - [x] 定义管理端 (Tauri) 接口: `docs/design/api_specs/admin_commands.md`。
    - [x] 定义 Web 端 (HTTP) 接口: `docs/requirements/web_api_specs.md` (最终版本)。

## [2026-01-11] Web Client 开发 (Done)
- **任务**: 构建移动端用户界面（Mock 数据驱动）。
- **进展**:
    - [x] 初始化 Vue 3 + TS + TailwindCSS v3 项目。
    - [x] 实现 Mock API Client (`src/api/client.ts`)，支持 `getActiveEvent`。
    - [x] 实现路由系统 (`vue-router`):
        - `/comment`: 留言页，自动感应活动状态。
        - `/result`: 结果页，轮询开奖结果。
    - [x] 实现 UI 组件:
        - `CommentForm`: 含手机号 JS 正则校验。
        - `WinnerDisplay`: 动效展示中奖信息。
        - `EventStatus`: 通用状态栏。
    - [x] 视觉优化: 添加过渡动画 (Transitions) 和加载状态。
    - [x] 验证: 手动测试 Mock 流程通过。

## [2026-01-11] Tauri 管理端前端开发 (Done)
- **任务**: 构建桌面管理后台（Mock 数据驱动）。
- **进展**:
    - [x] 创建 `src/api/admin.ts` 提供对 Tauri 命令的 Mock 支持。
    - [x] 实现 `ServerControl.vue`：控制 HTTP 服务器启动/停止。
    - [x] 实现 `EventManager.vue`：活动管理、查看留言、执行抽奖。
    - [x] 重构 `App.vue` 整合管理端布局。
    - [x] 验证: 管理端前端原型功能在 Mock 模式下运行正常。
