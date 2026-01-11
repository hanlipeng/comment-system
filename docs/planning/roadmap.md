| ID | 模块 | 依赖 | 优先级 | 状态 | 描述 |
|:---|:---|:---|:---|:---|:---|
| C1 | AdminInterface | - | P0 | Done | [契约] 管理后台接口定义 (Tauri Commands: 启动服务器, 创建活动, 抽奖) |
| C2 | WebInterface | - | P0 | Done | [契约] Web 端 HTTP API 规范 (POST /comment, GET /result) |
| S1 | ServerMgr | C1 | P1 | Pending | [实现] HTTP 服务器生命周期管理 (Axum 启动/停止控制) |
| S2 | EventLogic | C1, C2 | P1 | Pending | [实现] 核心业务逻辑：添加留言、随机抽奖、活动状态管理 |
| R1 | SqliteStore | S2 | P2 | Pending | [基础设施] SQLite 持久化存储实现 |
| UI1 | AdminApp | C1 | P1 | Done (Mock) | [界面] Tauri 管理后台 UI 开发，集成 Mock API |
| UI2 | WebApp | C2 | P1 | Done (Mock) | [界面] 用户移动端留言及结果展示页面，集成 Mock API |
