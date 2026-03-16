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

## [2026-01-12] Web 端功能扩展与大屏适配 (Done)
- **任务**: 增强 Web 端互动性，增加大屏展示页面。
- **进展**:
    - [x] **后端接口契约**: 
        - 在 `src-tauri/src/api/web.rs` 定义 `get_comments` 接口。
        - 添加 `WebComment` DTO（手机号脱敏）。
    - [x] **Web API Mock**: 
        - 更新 `client.ts`，支持 `getComments`。
        - 实现 Mock 数据生成（20+ 条随机留言）。
    - [x] **组件开发**:
        - 实现 `ScrollingCommentList.vue`：支持 CSS 动画无缝垂直滚动。
        - 支持 `size="large"` 模式，适配大屏阅读距离。
    - [x] **大屏适配 (DesktopResultView)**:
        - 新增 `/desktop` 路由及专用视图。
        - 采用全屏布局 (90vh 内容区)，超大字体适配投影展示。
        - 集成二维码生成器 (`qrcode.vue`)，动态指向留言页。
    - [x] **移动端同步**:
        - 在 `ResultView.vue` 结果页集成滚动留言预览。
    - [x] **修复**: 解决 `ScrollingCommentList.vue` 中 `v-if/v-else` 语法分组错误。

## [2026-01-12] Event Deletion (Backend)
- **任务**: 实现活动软删除功能的后端逻辑。
- **进展**:
    - [x] **Core**: 
        - 更新 `EventStatus` 枚举添加 `Deleted`。
        - 在 `EventServicePort` 添加 `delete_event` 接口。
        - 实现 `delete_event` 逻辑 (软删除)。
        - 更新 `list_events`, `get_event`, `get_active_event` 自动过滤已删除活动。
    - [x] **Admin API**: 实现 `admin_delete_event` 命令。
    - [x] **Web API**: 
        - 更新 `get_active_event` 和 `get_event` 处理 `Deleted` 状态映射。
    - [x] **Tests**: 
        - 修复 MockStorage 编译错误。
        - 添加 `test_delete_event` 单元测试并通过。
    - [ ] **Frontend**: 前端适配待办，详情见 `docs/requirements/micro/req_004_frontend_implementation.md`。

## [2026-01-13] Timestamp Standardization (Backend)
- **任务**: 统一所有 API 时间戳为毫秒级。
- **进展**:
    - [x] **Core Logic**: 更新 `create_event`, `add_comment`, `draw_winner`, `update_event_status` 中获取当前时间的方式为 `Utc::now().timestamp_millis()`。
    - [x] **Tests**: 添加 `test_timestamp_is_milliseconds` 验证时间戳格式。
    - [x] **Models/DB**: 确认 `i64` 类型和 SQLite `INTEGER` 可兼容毫秒级时间戳。

## [2026-01-13] Admin Network IP Link (Backend)
- **任务**: 实现获取局域网 IP 的功能，方便生成访问链接。
- **进展**:
    - [x] **Dependencies**: 引入 `local-ip-address` 库。
    - [x] **Admin API**: 实现 `get_network_ip` Tauri 命令，返回完整的 URL (例如 `http://192.168.1.5:3000`)。
    - [x] **Config**: 增强 `settings.toml` 初始化逻辑，支持生成带有注释的默认配置模板，并允许字段缺省（使用 Default trait）。
    - [x] **Integration**: 在 `src-tauri/src/lib.rs` 中注册新命令。
    - [x] **Tests**: 在 `tests/api/admin_tests.rs` 中添加集成测试验证 URL 格式。

## [2026-01-13] Wi-Fi API (Backend)
- **任务**: 实现从配置文件读取 Wi-Fi 信息并通过 API 暴露。
- **进展**:
    - [x] **Config**: 更新 `Settings` 结构体，添加 `wifi_ssid`, `wifi_password`, `wifi_encryption` 字段。
    - [x] **Web API**: 实现 `GET /api/v1/wifi` 接口，返回配置的 Wi-Fi 信息。
    - [x] **Refactor**: 重构 `HttpServer` 和 `api::web::router` 以支持注入 `Settings`。
    - [x] **Tests**: 添加 `tests/api/wifi_tests.rs` 进行接口测试。
