use crate::DATABASE;
use serde_json::{json, Value};
use url::form_urlencoded::Parse;

pub async fn find_player_id(params: &mut Parse<'_>) -> Value {
    match params.next() {
        Some(x) => {
            if x.0 == "username" {
                match DATABASE
                    .lock()
                    .await
                    .as_ref()
                    .unwrap()
                    .find_player_id(x.1.to_string())
                    .await
                {
                    Some(x) => json!({ "result": x }),
                    None => json!({"error":"Could not find user"}),
                }
            } else {
                json!({"error":"Did not specify \"username\" url paramiter"})
            }
        }
        None => json!({"error":"Did not specify \"username\" url paramiter"}),
    }
}

pub async fn find_player_name(params: &mut Parse<'_>) -> Value {
    match params.next() {
        Some(x) => {
            if x.0 == "id" {
                match x.1.to_string().parse::<i64>() {
                    Ok(id) => {
                        match DATABASE
                            .lock()
                            .await
                            .as_ref()
                            .unwrap()
                            .find_player_name(id)
                            .await
                        {
                            Some(x) => json!({ "result": x }),
                            None => json!({"error":"Could not find user"}),
                        }
                    }
                    Err(_) => json!({"error":"Id is not a number"}),
                }
            } else {
                json!({"error":"Did not specify \"id\" url paramiter"})
            }
        }
        None => json!({"error":"Did not specify \"id\" url paramiter"}),
    }
}
