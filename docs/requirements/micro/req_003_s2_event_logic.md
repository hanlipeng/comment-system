# 微观需求: S2 EventLogic

## 元数据
- **ID**: REQ-MICRO-003
- **关联宏观需求**: REQ-MACRO-001
- **模块**: S2 (Core Logic)
- **依赖**: 定义了 R1 (Storage) 的接口契约
- **优先级**: P1
- **状态**: Pending

## 目标
实现留言系统的核心业务逻辑，充当 `AdminInterface` (C1) 和 `WebInterface` (C2) 的后端控制器，并定义数据持久化层 `Storage` 的标准行为。

## 领域模型 (Core Domain Models)

### 1. Event (活动)
```rust
pub struct Event {
    pub id: String,         // UUID
    pub title: String,      // 活动标题
    pub status: EventStatus,// 活动状态
    pub created_at: i64,    // 创建时间戳
}

pub enum EventStatus {
    Active,
    Closed,
}
```

### 2. Comment (留言)
```rust
pub struct Comment {
    pub id: String,         // UUID
    pub event_id: String,   // 关联活动ID
    pub nickname: String,   // 用户昵称
    pub content: String,    // 留言内容
    pub phone: String,      // 手机号 (仅用于后台领奖联系)
    pub is_winner: bool,    // 是否为中奖者
    pub created_at: i64,    // 创建时间戳
}
```

### 3. PublicWinner (公开的中奖信息)
用于 Web 端展示，需脱敏。
```rust
pub struct PublicWinner {
    pub nickname: String,
    pub content: String,
    pub phone_masked: String, // e.g., "138****1234"
}
```

## 业务逻辑规范 (Service Interface)

`EventService` 结构体应提供以下方法供 C1 和 C2 调用：

### 1. 创建活动 (Create Event)
- **输入**: `title: String`
- **逻辑**:
  1. 生成 UUID。
  2. 设置状态为 `Active`。
  3. 记录 `created_at`。
  4. 调用 `Storage::create_event`。
- **验证**: `title` 不能为空且长度不超过 100 字符。

### 2. 添加留言 (Add Comment)
- **输入**: `event_id`, `nickname`, `content`, `phone`
- **逻辑**:
  1. 检查活动是否存在且状态为 `Active` (调用 `Storage::get_event`)。若不存在或已关闭，返回错误。
  2. 生成 UUID。
  3. 默认 `is_winner = false`。
  4. 调用 `Storage::add_comment`。
- **验证**: 
  - `nickname`: 非空，max 50 chars。
  - `content`: 非空，max 500 chars。
  - `phone`: 简单的格式校验 (11位数字)。

### 3. 抽奖 (Draw Winner)
- **输入**: `event_id`
- **逻辑**:
  1. 检查活动是否存在。
  2. 检查活动是否已有中奖者 (可选规则：是否允许重复抽奖？本期定义：**每个活动仅限一名中奖者**。若已有，需确认是否覆盖或报错。策略：**报错，提示需先重置或仅允许一次**)。
  3. 调用 `Storage::get_comments(event_id)` 获取候选池。
  4. 若候选池为空，返回错误。
  5. 使用随机数生成器 (RNG) 随机选择一个 `Comment`。
  6. 将该 `Comment` 的 `is_winner` 置为 `true`。
  7. 调用 `Storage::mark_winner(comment_id)` 更新存储。
  8. 可选：自动将活动状态设为 `Closed`? (本期策略：抽奖后**保持 Active** 或 **手动 Close**，由 Admin 决定，但通常抽奖意味着结束。暂定：抽奖仅标记中奖者，不副作用修改活动状态)。

### 4. 获取活动详情 (Get Event View)
- **输入**: `event_id`
- **逻辑**:
  1. 获取 Event 数据。
  2. (可选) 获取 Winner 数据。
  3. 组装为 Web 端需要的 DTO (包含脱敏后的 Winner 信息)。

## 依赖接口 (Downstream Contract)

S2 模块定义 `Storage` trait，R1 模块负责实现。
(参考 `req_002_r1_sqlite.md`，在此处正式确认 S2 的需求)

```rust
#[async_trait]
pub trait Storage: Send + Sync {
    // Event
    async fn create_event(&self, event: &Event) -> Result<(), StorageError>;
    async fn get_event(&self, id: &str) -> Result<Option<Event>, StorageError>;
    async fn list_events(&self) -> Result<Vec<Event>, StorageError>;
    
    // Comment
    async fn add_comment(&self, comment: &Comment) -> Result<(), StorageError>;
    async fn get_comments(&self, event_id: &str) -> Result<Vec<Comment>, StorageError>;
    // 标记中奖者，可能需要原子操作或事务，但在简单实现中只需更新字段
    async fn update_comment_winner(&self, comment_id: &str, is_winner: bool) -> Result<(), StorageError>;
    // 获取指定活动的中奖者 (便于快速查询)
    async fn get_event_winner(&self, event_id: &str) -> Result<Option<Comment>, StorageError>;
}
```

## 测试策略 (Test Plan)
1. **Mock Storage**: 在单元测试中实现一个 `MockStorage` (基于 `HashMap` 或 `Mockall`)。
2. **逻辑测试**:
   - 测试创建活动参数校验。
   - 测试向不存在的活动添加留言。
   - 测试抽奖逻辑（当没有留言时、当只有一条留言时）。
   - 测试随机性 (虽然单元测试难测随机，但可测试接口调用)。
