use tokio::sync::watch;
use tokio::time::{sleep, Duration};

async fn listener(mut rx: watch::Receiver<String>) {
    while rx.changed().await.is_ok() {
        let new_config = rx.borrow().clone();

        println!("Received new config: {}", new_config);
    }
}

// multi-producer, multi-consumer, no history kept
pub async fn watch_channel_example() {
    let config = String::from("config");

    let (tx, rx_1) = watch::channel(config);

    let rx_2 = tx.subscribe();

    tokio::spawn(listener(rx_1));
    tokio::spawn(listener(rx_2));

    tx.send("new_config".to_string()).unwrap();

    sleep(Duration::from_millis(500)).await;
}
