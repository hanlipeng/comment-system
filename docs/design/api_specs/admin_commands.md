# Admin API Specs (Tauri Commands)

## 1. 服务器控制
### `start_server`
- **功能**: 启动嵌入式 HTTP 服务器。
- **参数**:
    - `port: u16` (端口号)
- **返回**: `Result<(), String>`

### `stop_server`
- **功能**: 停止服务器。
- **返回**: `Result<(), String>`

## 2. 活动管理
### `create_event`
- **功能**: 创建一个新的留言活动。
- **参数**:
    - `title: String` (活动名称)
- **返回**: `Result<EventId, String>`

### `list_events`
- **功能**: 获取所有活动列表。
- **返回**: `Result<Vec<Event>, String>`

### `get_comments`
- **功能**: 获取指定活动的所有留言。
- **参数**:
    - `event_id: String`
- **返回**: `Result<Vec<Comment>, String>`

## 3. 抽奖逻辑
### `draw_winner`
- **功能**: 在指定活动中随机抽取一名中奖者。
- **参数**:
    - `event_id: String`
- **返回**: `Result<Comment, String>` (返回中奖的留言信息)
