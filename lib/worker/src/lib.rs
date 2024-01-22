use gloo_worker::Registrable;
use shared::console_log;
use shared::TestWorker;
use wasm_bindgen::prelude::*;

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(&format!($($t)*))))
}

#[wasm_bindgen(start)]
pub async fn start_worker() {
    console_log!("worker started");
    TestWorker::registrar().register();
}
