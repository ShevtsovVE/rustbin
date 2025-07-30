use redis::{RedisResult, AsyncCommands};
use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::str::FromStr;

#[derive(Clone)]
pub struct Storage {
    redis: redis::Client,
    db: sqlx::SqlitePool,
}

impl Storage {
    pub async fn new(redis_url: &str, db_url: &str) -> Self {
        let redis = redis::Client::open(redis_url).unwrap();
        
        let options = SqliteConnectOptions::from_str(db_url)
            .unwrap()
            .create_if_missing(true);
        
        let db = SqlitePool::connect_with(options).await.unwrap();
        
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS pastes (
                id TEXT PRIMARY KEY,
                created_at DATETIME DEFAULT CURRENT_TIMESTAMP
            )
            "#
        ).execute(&db).await.unwrap();

        Storage { redis, db }
    }

    pub async fn save_paste(&self, content: String) -> String {
        let id = nanoid::nanoid!(6);
        
        let mut conn = self.redis.get_async_connection().await.unwrap();
        let _: () = conn.set(&id, &content).await.unwrap();
        
        sqlx::query(
            "INSERT INTO pastes (id) VALUES (?)"
        )
        .bind(&id)
        .execute(&self.db)
        .await.unwrap();
        
        id
    }

    pub async fn get_paste(&self, id: &str) -> Option<String> {
        let mut conn = self.redis.get_async_connection().await.unwrap();
        let result: RedisResult<String> = conn.get(id).await;
        
        match result {
            Ok(content) => Some(content),
            Err(_) => None,
        }
    }
}