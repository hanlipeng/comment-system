# 微观需求: R1 SqliteStore

## 元数据
- **ID**: REQ-MICRO-002
- **关联宏观需求**: REQ-MACRO-001
- **模块**: R1 (Storage)
- **优先级**: P2
- **状态**: Pending

## 目标
使用 SQLite 作为持久化存储后端，替换原计划的内存存储。确保数据在应用重启后不丢失。

## 数据库设计 (Schema)

### 1. Events Table (`events`)
| 字段名 | 类型 | 约束 | 描述 |
|:---|:---|:---|:---|
| id | TEXT | PRIMARY KEY | UUID 字符串 |
| title | TEXT | NOT NULL | 活动标题 |
| status | TEXT | NOT NULL | 'active' 或 'closed' |
| created_at | INTEGER | NOT NULL | 创建时间戳 (Unix Timestamp) |

### 2. Comments Table (`comments`)
| 字段名 | 类型 | 约束 | 描述 |
|:---|:---|:---|:---|
| id | TEXT | PRIMARY KEY | UUID 字符串 |
| event_id | TEXT | NOT NULL | 外键 -> events.id |
| nickname | TEXT | NOT NULL | 用户昵称 |
| content | TEXT | NOT NULL | 留言内容 |
| phone | TEXT | NOT NULL | 手机号 |
| is_winner | BOOLEAN | DEFAULT 0 | 是否中奖 (0/1) |
| created_at | INTEGER | NOT NULL | 创建时间戳 |

## 接口需求 (Repository Trait)
Rust 核心逻辑层应定义 `Storage` trait，本模块负责实现该 trait。

```rust
#[async_trait]
pub trait Storage: Send + Sync {
    async fn create_event(&self, event: Event) -> Result<()>;
    async fn get_event(&self, id: &str) -> Result<Option<Event>>;
    async fn list_events(&self) -> Result<Vec<Event>>;
    
    async fn add_comment(&self, comment: Comment) -> Result<()>;
    async fn get_comments(&self, event_id: &str) -> Result<Vec<Comment>>;
    async fn mark_winner(&self, comment_id: &str) -> Result<()>;
}
```

## 技术选型建议
- **Crate**: `sqlx` (配合 `runtime-tokio`) 或 `rusqlite`。鉴于 Tauri 使用 Tokio，且可能需要异步 IO，推荐使用 `sqlx` (SQLite)。
- **Migration**: 使用简单的 SQL 脚本在启动时初始化表结构，或者使用 `sqlx-cli` 管理迁移。

## 验收标准
- [ ] 应用启动时自动创建 SQLite 数据库文件及表结构。
- [ ] 能成功插入和查询 Event 数据。
- [ ] 能成功插入和查询 Comment 数据，并关联 Event。
- [ ] 并发读写测试通过（SQLite 只有一把写锁，需注意锁争用，但本项目并发量预计较小，WAL 模式可优化读取）。
