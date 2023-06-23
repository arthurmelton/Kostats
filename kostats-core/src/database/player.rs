use crate::{database::{Database}, shared::player::Player};
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

        macro_rules! get_item {
            ( $player:expr, $key:expr ) => {
                {
                    $player
                        .clone()
                        .find(|x| x.key == $key)
                        .map(|x| x.value.parse::<i64>().unwrap())
                }
            };
        }

        Some(Player {
            name: self.find_player_name(id).await?,

            distance_glide: get_item!(player, "sn9.distance_glide"),
            distance_sprint: get_item!(player, "sn9.distance_sprint"),
            distance_walk: get_item!(player, "sn9.distance_walk"),
            distance_ballform: get_item!(player, "sn9.distance_ballform"),

            hit_given: get_item!(player, "sn9.unr.hit_given"),
            hit_received: get_item!(player, "sn9.unr.hit_received"),

            ko_given: get_item!(player, "sn9.unr.ko_given"),
            ko_given_doubles: get_item!(player, "sn9.unr.ko_given_doubles"),
            ko_given_frenzes: get_item!(player, "sn9.unr.ko_given_frenzes"),
            ko_given_triples: get_item!(player, "sn9.unr.ko_given_triples"),
            ko_received: get_item!(player, "sn9.unr.ko_received"),

            successful_tackles: get_item!(player, "sn9.unr.successful_tackles"),

            playtime: get_item!(player, "sn9.tko.playtime"),

            mvp: get_item!(player, "sn9.tko.match_mvp"),
            match_wins: get_item!(player, "sn9.tko.wins_match"),
            rounds_win: get_item!(player, "sn9.tko.wins_round"),
            rounds_lost: get_item!(player, "sn9.tko.losses_round"),
        })
    }
}
