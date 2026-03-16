# 前端 API 适配指南 (Frontend API Adaptation Guide)

由于后端 API (`src-tauri/src/api/web.rs`) 和逻辑 (`src-tauri/src/core/logic.rs`) 的更新，前端 `web-client` 需要进行相应的适配以确保功能正常。

## 1. 类型定义变更 (Type Definitions)

后端 `Event` 实体的状态字段增加了 `deleted` 状态。虽然前端可能主要关注 `active` 和 `closed`，但建议同步更新类型定义以防未来扩展或显示错误。

**建议修改文件**: `web-client/src/api/client.ts`

```typescript
// Old
export interface EventData {
  // ...
  status: 'active' | 'closed';
  // ...
}

// New
export interface EventData {
  // ...
  status: 'active' | 'closed' | 'deleted'; 
  // ...
}
```

## 2. 错误处理适配 (Error Handling)

提交留言接口 `POST /events/:id/comments` 现在的错误返回更加规范，前端应捕获这些状态码并向用户展示具体提示。

| HTTP 状态码 | 含义 | 建议前端提示 |
| :--- | :--- | :--- |
| **403 Forbidden** | 活动已结束 (Closed) | "活动已结束，停止接受新留言。" |
| **409 Conflict** | 手机号重复 | "该手机号已参与过本活动，请勿重复提交。" |
| **404 Not Found** | 活动不存在 | "活动不存在或已被删除。" |
| **400 Bad Request** | 参数错误 | "请填写所有必填字段。" |

**建议修改逻辑**: `api.postComment` 方法

```typescript
// 伪代码示例
try {
  await fetch(...);
} catch (error) {
  if (response.status === 409) {
    throw new Error("该手机号已参与过本活动");
  } else if (response.status === 403) {
    throw new Error("活动已结束");
  }
  // ... 其他错误处理
}
```

## 3. 接口调用规范

请确保所有 API 调用遵循最新的 OpenAPI 文档：`docs/design/api_specs/web_api.yaml`。

- **GET /events/active**: 仅返回当前活跃的活动。如果没有活跃活动，返回 `null` (200 OK)。
- **GET /events/:id/comments**: 默认返回最新的 20 条留言，按时间倒序。前端无需再在客户端进行切片处理（除非为了 Mock 效果）。
