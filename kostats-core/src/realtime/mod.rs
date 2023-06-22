use fred::{monitor, prelude::*};
use serde::Serialize;
use serde_json::Value;

use async_stream::stream;
use futures_core::stream::Stream;
use futures_util::StreamExt;

use crate::database::Database;

pub struct Realtime {
    database: Database,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(content = "result")]
pub enum Item {
    LevelUp { username: String, xp: u64 },

    End,
}

#[derive(Debug, Clone)]
pub struct Command {
    pub command: String,
    pub effect: String,
    pub value: Value,
}

impl Command {
    pub fn from_redis(command: monitor::Command) -> Option<Command> {
        Some(Command {
            command: command.command,
            effect: command.args.first()?.as_string()?,
            value: serde_json::from_str(
                &snailquote::unescape(&format!("\"{}\"", command.args.get(1)?.as_string()?))
                    .ok()?,
            )
            .ok()?,
        })
    }
}

impl Realtime {
    pub fn listen(
        posgres_path: String,
        redis_path: String,
    ) -> impl Stream<Item = anyhow::Result<Item>> {
        stream! {
            let config = RedisConfig::from_url(&format!("redis://{redis_path}"))?;
            let mut monitor_stream = monitor::run(config).await?;

            let selfs = Realtime {
                database: Database::new(posgres_path).await?
            };

            while let Some(command) = monitor_stream.next().await {
                if let Some(command) = Command::from_redis(command) {
                    for i in [selfs.player_update(command.clone()).await].into_iter().flatten() {
                        yield Ok(i);
                    }
                }
            }
            yield Ok(Item::End);
        }
    }
}

pub mod player;
