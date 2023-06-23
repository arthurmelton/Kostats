use crate::realtime::{to_int, Command, Item, Realtime};
use crate::shared::player::Player;

impl Realtime {
    pub async fn player_update(&self, message: Command) -> Option<Item> {
        if message.command == "PUBLISH" && message.effect.starts_with("socket:user:") {
            let player = message.effect[12..].to_string();
            return match message.value["type"].as_str()? {
                //"_street_rank_update" => self.rank_update(player, message).await,
                "_persistence_get_user_pairs" => self.match_done(player, message).await,
                _ => None,
            };
        }

        None
    }

    //pub async fn rank_update(&self, player_string: String, message: Command) -> Option<Item> {
    //    let player = self
    //        .database
    //        .find_player_name(player_string.parse::<i64>().ok()?)
    //        .await?;
    //    let skill = message.value["update"][player_string].as_u64()?;

    //    Some(Item::LevelUp {
    //        username: player,
    //        xp: skill,
    //    })
    //}

    pub async fn match_done(&self, player_string: String, message: Command) -> Option<Item> {
        match message.value["full_update"].as_bool()? {
            false => {
                let player_number = player_string.parse::<i64>().ok()?;
                let player = self.database.find_player_name(player_number).await?;
                for i in message.value["users"].as_array()? {
                    if i["user_id"]["velan"].as_i64()? == player_number {
                        let pairs = &i["pairs"];
                        let game = Box::new(Player {
                            name: player.clone(),

                            distance_glide: to_int(pairs["lt.distance_glide"].clone()),
                            distance_sprint: to_int(pairs["lt.distance_walk"].clone()),
                            distance_walk: to_int(pairs["lt.distance_walk"].clone()),
                            distance_ballform: to_int(pairs["lt.distance_ballform"].clone()),

                            hit_given: to_int(pairs["lt.unr.hit_given"].clone()),
                            hit_received: to_int(pairs["lt.unr.hit_received"].clone()),

                            ko_given: to_int(pairs["lt.unr.ko_given"].clone()),
                            ko_given_doubles: to_int(pairs["lt.unr.ko_given_doubles"].clone()),
                            ko_given_frenzes: to_int(pairs["lt.unr.ko_given_frenzes"].clone()),
                            ko_given_triples: to_int(pairs["lt.unr.ko_given_triples"].clone()),
                            ko_received: to_int(pairs["lt.unr.ko_received"].clone()),

                            successful_tackles: to_int(pairs["lt.unr.successful_tackles"].clone()),

                            playtime: to_int(pairs["lt.tko.playtime"].clone()),

                            mvp: to_int(pairs["lt.tko.match_mvp"].clone()),
                            match_wins: to_int(pairs["lt.tko.wins_match"].clone()),
                            rounds_win: to_int(pairs["lt.tko.wins_round"].clone()),
                            rounds_lost: to_int(pairs["lt.tko.losses_round"].clone()),
                        });
                        let total = Box::new(Player {
                            name: player,

                            distance_glide: to_int(pairs["sn9.distance_glide"].clone()),
                            distance_sprint: to_int(pairs["sn9.distance_walk"].clone()),
                            distance_walk: to_int(pairs["sn9.distance_walk"].clone()),
                            distance_ballform: to_int(pairs["sn9.distance_ballform"].clone()),

                            hit_given: to_int(pairs["sn9.unr.hit_given"].clone()),
                            hit_received: to_int(pairs["sn9.unr.hit_received"].clone()),

                            ko_given: to_int(pairs["sn9.unr.ko_given"].clone()),
                            ko_given_doubles: to_int(pairs["sn9.unr.ko_given_doubles"].clone()),
                            ko_given_frenzes: to_int(pairs["sn9.unr.ko_given_frenzes"].clone()),
                            ko_given_triples: to_int(pairs["sn9.unr.ko_given_triples"].clone()),
                            ko_received: to_int(pairs["sn9.unr.ko_received"].clone()),

                            successful_tackles: to_int(pairs["sn9.unr.successful_tackles"].clone()),

                            playtime: to_int(pairs["sn9.tko.playtime"].clone()),

                            mvp: to_int(pairs["sn9.tko.match_mvp"].clone()),
                            match_wins: to_int(pairs["sn9.tko.wins_match"].clone()),
                            rounds_win: to_int(pairs["sn9.tko.wins_round"].clone()),
                            rounds_lost: to_int(pairs["sn9.tko.losses_round"].clone()),
                        });
                        return Some(Item::MatchResult { game, total });
                    }
                }
                None
            }
            true => None,
        }
    }
}
