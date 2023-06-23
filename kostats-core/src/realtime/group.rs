use crate::realtime::{Command, Item, Realtime};

impl Realtime {
    pub async fn group_update(&self, message: Command) -> Option<Item> {
        if message.command == "PUBLISH" && message.effect.starts_with("socket:user:") {
            let player = message.effect[12..].to_string();
            return match message.value["relay_type"].as_str()? {
                "_relay_group_invite" => {
                    Some(Item::GroupInvite {
                        receiver: self.database.find_player_name(player.parse().ok()?).await?,
                        sender: self.database.find_player_name(message.value["payload"]["sender_id"]["velan"].as_i64()?).await?
                    })
                },
                _ => None,
            };
        }
        None
    }
}
