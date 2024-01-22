use futures::{sink::SinkExt, StreamExt};
use gloo_worker::Spawnable;
use shared::console_log;
use shared::TestWorker;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn start_app() {
    console_log!("started main app");
    // worker js is generated by wasm-pack build --no-modules
    let mut bridge = TestWorker::spawner().spawn("./worker.js");
    console_log!("sending");
    bridge.send(2).await.unwrap();
    console_log!("sent");
    let result = bridge.next().await.unwrap();
    console_log!("result: {}", result);
}