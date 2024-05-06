use tokio::sync::broadcast;
use tokio::sync::broadcast::Sender;

async fn wait_notification(sender: Sender<String>, name: String) {
    let mut receiver = sender.subscribe();

    while let Ok(message) = receiver.recv().await {
        println!("{} received a message: {}", name, message);
    }
}

// multi producer, multi consumer, history kept
pub async fn broadcast_example() {
    let (tx, _rx) = broadcast::channel(100);

    tokio::spawn(
        wait_notification(tx.clone(), "John".to_string())
    );
    tokio::spawn(
        wait_notification(tx.clone(), "Jessica".to_string())
    );
    tokio::spawn(
        wait_notification(tx.clone(), "Eugene".to_string())
    );

    for i in 0..100 {
        tx.send(format!("Message {}", i).to_string()).unwrap();
    }
}
