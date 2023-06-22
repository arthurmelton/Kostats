use std::sync::{Arc, Mutex};
use std::thread;
use ws::listen;

mod request;
mod websocket;

use kostats_core::database::Database;
use request::DATABASE;

use websocket::{Server, CONNECTION};

pub async fn host(port: u16, db: String, redis: String) {
    *DATABASE.lock().await = Database::new(db.clone()).await.ok();

    thread::spawn(move || {
        listen(format!("0.0.0.0:{port}"), |out| Server {
            out,
            rx: Arc::new(Mutex::new(CONNECTION.lock().unwrap().add_rx())),
            open: Arc::new(Mutex::new(true)),
        })
        .unwrap();
    });

    websocket::listen_websocket(db, redis).await;
}
