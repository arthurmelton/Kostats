use crate::realtime::{Command, Item, Realtime};

impl Realtime {
    pub async fn player_update(&self, message: Command) -> Option<Item> {
        if message.command == "PUBLISH" && message.effect.starts_with("socket:user:") {
            let player = message.effect[12..].to_string();
            return match message.value["type"].as_str()? {
                "_street_rank_update" => self.rank_update(player, message).await,
                _ => None,
            };
        }

        None
    }

    pub async fn rank_update(&self, player_string: String, message: Command) -> Option<Item> {
        let player = self
            .database
            .find_player_name(player_string.parse::<i64>().ok()?)
            .await?;
        let skill = message.value["update"][player_string].as_u64()?;

        Some(Item::LevelUp {
            username: player,
            xp: skill,
        })
    }
}
