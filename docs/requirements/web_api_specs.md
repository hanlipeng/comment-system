# Web 客户端接口需求规范 (Web API Specs)

本文档定义了留言板 Web 前端与后端 HTTP 服务器之间的通信协议。

## 1. 基础信息
- **协议**: HTTP/1.1
- **数据格式**: JSON
- **基础路径**: `/api/v1`

## 2. 数据模型

### 活动对象 (Event)
| 字段 | 类型 | 说明 |
| :--- | :--- | :--- |
| `id` | String | 活动唯一标识 (UUID) |
| `title` | String | 活动标题 |
| `status` | String | 状态: `active` (进行中), `closed` (已结束) |
| `winner` | Object/Null | 中奖者信息，未开奖时为 `null` |

### 中奖者对象 (Winner)
| 字段 | 类型 | 说明 |
| :--- | :--- | :--- |
| `nickname` | String | 中奖者昵称 |
| `content` | String | 中奖留言内容 |
| `phone_masked` | String | 脱敏后的手机号 (例: 138****1234) |

---

## 3. 接口列表

### 3.1 获取当前活跃活动
> 用于页面初始化，当 URL 中未指定 `id` 时调用。

- **URL**: `/events/active`
- **方法**: `GET`
- **返回示例**:
  ```json
  {
    "id": "event-001",
    "title": "2026新年幸运抽奖",
    "status": "active",
    "winner": null
  }
  ```

### 3.2 获取指定活动详情
> 用于 `/comment` 或 `/result` 页面获取详情及状态轮询。

- **URL**: `/events/:id`
- **方法**: `GET`
- **参数**: `id` (路径参数)
- **返回示例 (已开奖)**:
  ```json
  {
    "id": "event-001",
    "title": "2026新年幸运抽奖",
    "status": "closed",
    "winner": {
      "nickname": "张三",
      "content": "祝大家新年快乐！",
      "phone_masked": "138****1234"
    }
  }
  ```

### 3.3 提交留言
> 用户在 `/comment` 页面提交表单。

- **URL**: `/events/:id/comments`
- **方法**: `POST`
- **请求体 (Body)**:
  ```json
  {
    "nickname": "小明",
    "content": "抽我抽我！",
    "phone": "13912345678"
  }
  ```
- **成功返回**: `201 Created`
- **失败返回**:
  - `400 Bad Request`: 格式错误或手机号不合法。
  - `403 Forbidden`: 活动已关闭，无法留言。
  - `404 Not Found`: 活动不存在。

---

## 4. 注意事项
1. **跨域 (CORS)**: 后端需允许来自 Web 端域名的跨域请求。
2. **频率限制**: 建议对同一手机号或 IP 的留言频率进行限制。
3. **安全性**: `phone` 字段仅在提交时传输，`GET` 接口中只能返回脱敏后的 `phone_masked`。
