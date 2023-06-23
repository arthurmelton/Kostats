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

    pub async fn find_player_xp(&self, id: i64) -> Option<i32> {
        Some(
            query!("SELECT * FROM public.street_rank WHERE user_id = $1", id)
                .fetch_one(&self.pool)
                .await
                .ok()?
                .raw_xp,
        )
    }

    pub async fn find_player_xp_touple(&self, id: i64) -> Option<(i32, i32)> {
        let xp = query!("SELECT * FROM public.street_rank WHERE user_id = $1", id)
            .fetch_one(&self.pool)
            .await
            .ok()?;
        let level = query!(
            "SELECT * FROM public.street_rank_rewards_season_6 WHERE total_xp = $1",
            xp.last_rewarded_xp_s6
        )
        .fetch_one(&self.pool)
        .await
        .ok()?
        .level?;
        Some((level, xp.raw_xp_s6 - xp.last_rewarded_xp_s6))
    }
}
