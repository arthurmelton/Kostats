use crate::{database::Database, shared::player::Player};
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
                .raw_xp_s6,
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

    pub async fn get_player(&self, id: i64) -> Option<Player> {
        let query = query!("SELECT * FROM key_value_pairs WHERE user_id = $1", id)
            .fetch_all(&self.pool)
            .await
            .ok()?;
        let player = query.iter();

        Some(Player {
            name: self.find_player_name(id).await?,

            distance_glide: player.clone().find(|x| x.key == "sn9.distance_glide").map(|x| x.value.parse::<i64>().unwrap()),
            distance_sprint: player.clone().find(|x| x.key == "sn9.distance_sprint").map(|x| x.value.parse::<i64>().unwrap()),
            distance_walk: player.clone().find(|x| x.key == "sn9.distance_walk").map(|x| x.value.parse::<i64>().unwrap()),
            distance_ballform: player.clone().find(|x| x.key == "sn9.distance_ballform").map(|x| x.value.parse::<i64>().unwrap()),

            hit_given: player.clone().find(|x| x.key == "sn9.unr.hit_given").map(|x| x.value.parse::<i64>().unwrap()),
            hit_received: player.clone().find(|x| x.key == "sn9.unr.hit_received").map(|x| x.value.parse::<i64>().unwrap()),

            ko_given: player.clone().find(|x| x.key == "sn9.unr.ko_given").map(|x| x.value.parse::<i64>().unwrap()),
            ko_given_doubles: player.clone().find(|x| x.key == "sn9.unr.ko_given_doubles").map(|x| x.value.parse::<i64>().unwrap()),
            ko_given_frenzes: player.clone().find(|x| x.key == "sn9.unr.ko_given_frenzes").map(|x| x.value.parse::<i64>().unwrap()),
            ko_given_triples: player.clone().find(|x| x.key == "sn9.unr.ko_given_triples").map(|x| x.value.parse::<i64>().unwrap()),
            ko_received: player.clone().find(|x| x.key == "sn9.unr.ko_received").map(|x| x.value.parse::<i64>().unwrap()),

            successful_tackles: player.clone().find(|x| x.key == "sn9.unr.successful_tackles").map(|x| x.value.parse::<i64>().unwrap()),

            playtime: player.clone().find(|x| x.key == "sn9.tko.playtime").map(|x| x.value.parse::<i64>().unwrap()),

            mvp: player.clone().find(|x| x.key == "sn9.tko.match_mvp").map(|x| x.value.parse::<i64>().unwrap()),
            match_wins: player.clone().find(|x| x.key == "sn9.tko.wins_match").map(|x| x.value.parse::<i64>().unwrap()),
            rounds_win: player.clone().find(|x| x.key == "sn9.tko.wins_round").map(|x| x.value.parse::<i64>().unwrap()),
            rounds_lost: player.clone().find(|x| x.key == "sn9.tko.losses_round").map(|x| x.value.parse::<i64>().unwrap()),
        })
    }
}
