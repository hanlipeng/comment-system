use anyhow::{Result, Context};
use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, migrate::MigrateDatabase};

#[derive(Clone, Debug)]
pub struct SqliteConnection {
    pub pool: Pool<Sqlite>,
}

impl SqliteConnection {
    /// 初始化连接池并确保数据库文件存在
    pub async fn connect(database_url: &str) -> Result<Self> {
        // 如果不是内存数据库且不存在，则创建文件
        if !database_url.contains(":memory:") && 
           !Sqlite::database_exists(database_url).await.unwrap_or(false) {
            Sqlite::create_database(database_url).await?;
        }

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await
            .context("Failed to connect to SQLite database")?;

        let connection = Self { pool };
        connection.init_tables().await?;
        Ok(connection)
    }

    /// 初始化表结构
    async fn init_tables(&self) -> Result<()> {
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS events (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                status TEXT NOT NULL,
                winner_comment_id TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS comments (
                id TEXT PRIMARY KEY,
                event_id TEXT NOT NULL,
                nickname TEXT NOT NULL,
                content TEXT NOT NULL,
                phone TEXT NOT NULL,
                is_winner BOOLEAN DEFAULT 0,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL,
                FOREIGN KEY(event_id) REFERENCES events(id)
            );
            "#,
        )
        .execute(&self.pool)
        .await
        .context("Failed to initialize database tables")?;
        Ok(())
    }
}
