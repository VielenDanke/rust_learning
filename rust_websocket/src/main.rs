use rust_websocket;

#[tokio::main]
async fn main() {
    rust_websocket::run_server().await;
}
