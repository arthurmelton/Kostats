use crate::realtime::{Command, Item, Realtime};
use serde_json::Value;

impl Realtime {
    pub async fn matchmake_update(&self, message: Command) -> Option<Item> {
        if message.effect.starts_with("user:mm:") {
            let player = self
                .database
                .find_player_name(message.effect[8..].to_string().parse::<i64>().ok()?)
                .await?;
            return if message.command == "HSET"
                && message.value["mm_mmr"] == Value::String("2500".to_string())
            {
                Some(Item::JoinMatchMake { username: player })
            }
            //else if message.command == "HGETALL" {
            //    println!("{:?}", message.value);
            //    Some(Item::LeaveMatchMake {
            //        username: player
            //    })
            //}
            else {
                None
            };
        }

        None
    }
}
