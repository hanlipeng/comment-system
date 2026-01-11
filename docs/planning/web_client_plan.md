# Web Client 开发计划

## 目标
构建一个移动端友好的 Web 页面，用于用户提交留言和查看抽奖结果。

## 技术栈
- **框架**: Vue 3 + TypeScript (Vite)
- **样式**: TailwindCSS (或手写 CSS 以保持轻量)
- **HTTP**: Fetch API / Axios

## 任务分解 (Task Breakdown)

### Phase 1: 初始化 (Setup)
- [ ] **W1**: 初始化 Vite 项目到 `web-client/` 目录。
- [ ] **W2**: 配置 TailwindCSS。
- [ ] **W3**: 建立 API 适配层 (`src/api/client.ts`)，包含 Mock 模式开关。

### Phase 2: 组件开发 (Components)
- [ ] **W4 (UI)**: 开发 `CommentForm` 组件
    - 输入: 昵称, 内容, 手机号 (校验: 必填, 格式)。
    - 状态: 提交中, 成功, 失败。
- [ ] **W5 (UI)**: 开发 `WinnerDisplay` 组件
    - 展示: 昵称, 内容, 脱敏手机号。
    - 动效: 简单的出现动画。
- [ ] **W6 (UI)**: 开发 `EventStatus` 组件
    - 展示: 活动标题, "活动进行中" / "已结束"。

### Phase 3: 页面与逻辑 (Logic)
- [ ] **W7 (Page)**: 组装 `App.vue` 或 `EventPage.vue`。
    - 逻辑: 加载时获取活动信息 (`GET /events/:id`)。
    - 逻辑: 提交表单 (`POST ...`)。
    - 逻辑: 轮询 (Polling) 检查活动状态 (每 5-10秒)。

### Phase 4: 验证 (Verification)
- [ ] **W8**: 运行前端测试 (Unit/Component Test)。
- [ ] **W9**: 手动验证 Mock 数据流程。

## 接口依赖 (Mock First)
在后端就绪前，前端将拦截请求并返回以下 Mock 数据：
- `GET /events/1`: `{ id: "1", title: "年会抽奖", status: "active", winner: null }`
- `POST /comments`: `201 Created`
