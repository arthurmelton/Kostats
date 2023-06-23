use crate::DATABASE;
use serde_json::{json, Value};
use url::form_urlencoded::Parse;

pub async fn find_player_id(params: &mut Parse<'_>) -> Value {
    default_req!(params, "username", find_player_id)
}

pub async fn find_player_name(params: &mut Parse<'_>) -> Value {
    default_req!(params, "id", find_player_name)
}

pub async fn find_player_xp(params: &mut Parse<'_>) -> Value {
    default_req!(params, "id", find_player_xp)
}

pub async fn find_player_xp_touple(params: &mut Parse<'_>) -> Value {
    default_req!(params, "id", find_player_xp_touple)
}

pub async fn get_player(params: &mut Parse<'_>) -> Value {
    default_req!(params, "id", get_player)
}
