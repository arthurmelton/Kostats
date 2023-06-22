use crate::database::Database;
use sqlx::query;

impl Database {
    pub async fn find_player_name(&self, id: i64) -> Option<String> {
        Some(
            query!("SELECT * FROM users WHERE id = $1", id)
                .fetch_one(&self.pool)
                .await
                .ok()?
                .username,
        )
    }

    pub async fn find_player_id(&self, name: String) -> Option<i64> {
        Some(
            query!("SELECT * FROM users WHERE username = $1", name)
                .fetch_one(&self.pool)
                .await
                .ok()?
                .id,
        )
    }
}
