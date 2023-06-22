use bus::{Bus, BusReader};
use futures_util::{pin_mut, StreamExt};
use kostats_core::realtime::{Item, Realtime};
use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use std::thread;
use tokio::runtime::Runtime;
use ws::{CloseCode, Handler, Handshake, Request, Response, Result, Sender};

lazy_static! {
    pub static ref CONNECTION: Mutex<Bus<Item>> = Mutex::new(Bus::new(1000));
}

pub struct Server {
    pub out: Sender,
    pub rx: Arc<Mutex<BusReader<Item>>>,
    pub open: Arc<Mutex<bool>>,
}

impl Handler for Server {
    fn on_request(&mut self, req: &Request) -> Result<Response> {
        match req.resource() {
            "/ws" => Response::from_request(req),
            _ => Ok(
                Runtime::new()?.block_on(async { crate::request::handle(req.resource()).await })
            ),
        }
    }

    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let rx = self.rx.clone();
        let out = self.out.clone();
        let open = self.open.clone();
        thread::spawn(move || loop {
            if let Ok(text) = serde_json::to_string(&rx.lock().unwrap().recv().unwrap()) {
                if !*open.lock().unwrap() {
                    break;
                }
                let _ = out.send(text);
            }
        });
        Ok(())
    }

    fn on_close(&mut self, _: CloseCode, _: &str) {
        *self.open.lock().unwrap() = false;
    }
}

pub async fn listen_websocket(db: String, redis: String) {
    let stream = Realtime::listen(db, redis);
    pin_mut!(stream);

    while let Some(item) = stream.next().await {
        if let Ok(item) = item {
            CONNECTION.lock().unwrap().broadcast(item);
        }
    }
}
