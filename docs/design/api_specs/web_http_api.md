# Web HTTP API 规范 (WebInterface)

## 基础路径: `/api/v1`

## 1. 提交留言
### `POST /events/:id/comments`
- **功能**: 用户提交留言。
- **Payload**:
    ```json
    {
      "nickname": "String", // 昵称
      "content": "String",  // 留言内容
      "phone": "String"     // 手机号 (用于领奖联系)
    }
    ```
- **返回**: 
    - `201 Created`: 提交成功
    - `400 Bad Request`: 参数错误
    - `404 Not Found`: 活动不存在

## 2. 获取活动及结果
### `GET /events/:id`
- **功能**: 获取活动信息及抽奖结果（如果有）。
- **返回**:
    ```json
    {
      "id": "String",
      "title": "String",
      "status": "active | closed",
      "winner": {
        "nickname": "String",
        "content": "String",
        "phone_masked": "String" // 脱敏手机号 (如 138****1234)
      } | null
    }
    ```
