pub mod models;
pub mod logic;

pub use models::*;
pub use logic::*;

use async_trait::async_trait;
use uuid::Uuid;
use anyhow::Result;

#[async_trait]
pub trait EventServicePort: Send + Sync {
    /// 创建新活动
    async fn create_event(&self, title: String) -> Result<Event>;
    
    /// 获取指定活动
    async fn get_event(&self, event_id: Uuid) -> Result<Option<Event>>;
    
    /// 获取当前活跃活动
    async fn get_active_event(&self) -> Result<Option<Event>>;

    /// 添加留言
    async fn add_comment(&self, event_id: Uuid, nickname: String, content: String, phone: String) -> Result<Comment>;

    /// 获取最新留言
    async fn get_recent_comments(&self, event_id: Uuid, limit: usize) -> Result<Vec<Comment>>;

    /// 随机抽奖
    async fn draw_winner(&self, event_id: Uuid) -> Result<Comment>;

    /// 获取活动脱敏后的中奖信息
    async fn get_public_winner(&self, event_id: Uuid) -> Result<Option<PublicWinner>>;

    /// [Admin] 获取所有活动列表
    /// 
    /// 返回所有创建过的活动，通常按创建时间倒序排列。
    async fn list_events(&self) -> Result<Vec<Event>>;

    /// [Admin] 获取指定活动的所有留言
    /// 
    /// 返回指定 `event_id` 下的所有留言。
    /// 注意：此接口返回的数据包含完整手机号，仅供管理员使用。
    async fn list_comments(&self, event_id: Uuid) -> Result<Vec<Comment>>;

    /// [Admin] 更新活动状态
    /// 
    /// 用于管理员手动控制活动的开启与关闭。
    async fn update_event_status(&self, event_id: Uuid, status: EventStatus) -> Result<()>;

    /// [Admin] 删除活动 (软删除)
    /// 
    /// 将活动标记为 `Deleted` 状态，使其在所有正常查询中不可见。
    async fn delete_event(&self, event_id: Uuid) -> Result<()>;
}