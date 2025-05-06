pub mod request_handler;
pub use std::net::TcpListener;

use std::sync::Arc;
use log::{error, info};
use crate::AppState;
use crate::http_server::request_handler::handle_request;


const ADDR: &str = "localhost:8000";

pub async fn start(app_state: AppState) {
    let Ok(listener) = TcpListener::bind(ADDR) else { error!("couldn't open {ADDR}"); return; };
    info!("Listening on: {ADDR}");
    let app_state = Arc::new(app_state);
    let pool = tokio::runtime::Builder::new_multi_thread().enable_time().enable_io().build().unwrap();

    for stream in listener.incoming() {
        let Ok(stream) = stream else { error!("failed to estabilish connection"); continue; };
        let state_clone = app_state.clone();

        pool.spawn(async move {
            info!("connection estabilished");
            handle_request(stream, state_clone).await
        });
    }
}
