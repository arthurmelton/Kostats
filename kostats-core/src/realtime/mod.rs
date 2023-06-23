use fred::{monitor, prelude::*};
use serde::Serialize;
use serde_json::{Map, Value};

use async_stream::stream;
use futures_core::stream::Stream;
use futures_util::StreamExt;

use crate::database::Database;
use crate::shared::player::Player;

pub struct Realtime {
    database: Database,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type")]
#[serde(content = "result")]
pub enum Item {
    LevelUp {
        username: String,
        xp: u64,
    },

    PlayerJoin {
        username: String,
    },
    PlayerLeave {
        username: String,
    },

    GroupInvite {
        sender: String,
        receiver: String,
    },

    JoinMatchMake {
        username: String,
    },
    LeaveMatchMake {
        username: String,
    },
    MatchResult {
        game: Box<Player>,
        total: Box<Player>,
    },

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
            value: {
                match command.args.len() {
                    1 => Value::Null,
                    2 => serde_json::from_str(&unescape(&command.args.get(1).unwrap().as_str()?)?)
                        .ok()?,
                    _ => {
                        if (command.args.len() - 1) % 2 == 0 {
                            let mut empty = Map::new();
                            command.args[1..].chunks(2).for_each(|x| {
                                empty.insert(
                                    (*x.first().unwrap().as_str().unwrap()).to_string(),
                                    Value::String(
                                        (*x.last().unwrap().as_str().unwrap()).to_string(),
                                    ),
                                );
                            });
                            Value::Object(empty)
                        } else {
                            Value::Null
                        }
                    }
                }
            },
        })
    }
}

fn unescape(text: &str) -> Option<String> {
    snailquote::unescape(&format!("\"{text}\"")).ok()
}

fn to_int(input: Value) -> Option<i64> {
    input.as_str()?.parse::<i64>().ok()
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
                    for i in [
                        selfs.player_update(command.clone()).await,
                        selfs.matchmake_update(command.clone()).await,
                        selfs.group_update(command.clone()).await
                    ].into_iter().flatten() {
                        yield Ok(i);
                    }
                }
            }
            yield Ok(Item::End);
        }
    }
}

pub mod group;
pub mod matchmake;
pub mod player;
