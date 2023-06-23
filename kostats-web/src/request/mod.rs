use kostats_core::database::Database;
use lazy_static::lazy_static;
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use url::Url;
use urlencoding::decode;
use ws::Response;

const ERROR: &[u8] = b"{\"error\":\"Endpoint was not found\"}";

lazy_static! {
    pub static ref DATABASE: Arc<Mutex<Option<Database>>> = Arc::new(Mutex::new(None));
}

pub async fn handle(path: &str) -> Response {
    let mut response = match make_request(path).await {
        Some(x) => Response::new(
            200,
            "OK",
            serde_json::to_string(&x)
                .unwrap()
                .bytes()
                .collect::<Vec<u8>>(),
        ),
        None => Response::new(404, "Not Found", ERROR.to_vec()),
    };
    response.headers_mut().push((
        "Content-Type".to_string(),
        "application/json".bytes().collect::<Vec<u8>>(),
    ));
    response
}

pub async fn make_request(path: &str) -> Option<Value> {
    let path = Url::parse(&format!("a://a{}", decode(path).ok()?)).ok()?;
    match path.path() {
        "/api/find_player_id" => Some(player::find_player_id(&mut path.query_pairs()).await),
        "/api/find_player_name" => Some(player::find_player_name(&mut path.query_pairs()).await),
        "/api/find_player_xp" => Some(player::find_player_xp(&mut path.query_pairs()).await),
        "/api/find_player_xp_touple" => {
            Some(player::find_player_xp_touple(&mut path.query_pairs()).await)
        }
        "/api/get_player" => Some(player::get_player(&mut path.query_pairs()).await),
        _ => None,
    }
}

pub mod player;
