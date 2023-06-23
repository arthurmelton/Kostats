use crate::DATABASE;
use serde_json::{json, Value};
use url::form_urlencoded::Parse;

default_req!("username", find_player_id);
default_req!("id", find_player_name);
default_req!("id", find_player_xp);
default_req!("id", find_player_xp_touple);
default_req!("id", get_player);
