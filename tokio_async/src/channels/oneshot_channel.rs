use tokio::sync::oneshot;
use tokio::sync::oneshot::{Receiver, Sender};

async fn receive_message(mut rx: Receiver<String>) {
    while let Ok(message) = rx.try_recv() {
        println!("Received message: {message}");
    }
}

async fn produce_message(tx: Sender<String>, message: String) {
    tx.send(message).unwrap();
}

// single producer single consumer
pub async fn oneshot_example() {
    let (tx, rx) = oneshot::channel();

    let send_handle = tokio::spawn(
        produce_message(tx, "Message".to_string())
    );
    let receive_handle = tokio::spawn(
        receive_message(rx)
    );
    send_handle.await.unwrap();
    receive_handle.await.unwrap();
}
