#[macro_export]
macro_rules! console_log {
    ($($t:tt)*) =>
        (web_sys::console::log_1(
            &wasm_bindgen::JsValue::from_str(&format!($($t)*))))
}

use futures::{sink::SinkExt, StreamExt};
use gloo_worker::reactor::{reactor, ReactorScope};

#[reactor]
pub async fn TestWorker(mut scope: ReactorScope<u32, u32>) {
    console_log!("TestWorker function triggered");
    while let Some(m) = scope.next().await {
        // just add the input to itself
        if scope.send(m + m).await.is_err() {
            break;
        }
    }
}
