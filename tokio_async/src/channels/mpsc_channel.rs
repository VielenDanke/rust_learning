use rand::random;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

async fn send(sender: Sender<String>) {
    for i in 0..100 {
        sender.send(format!("Thread name: {}, Message {}", format!("# {}", random::<i32>()).to_string(), i).to_string()).await.unwrap();
    }
}

// multiple producers single consumer
pub async fn mpsc_example() {
    let (sender, mut receiver) = mpsc::channel(32);

    let sender1 = sender.clone();

    tokio::spawn(send(sender));

    tokio::spawn(send(sender1));

    while let Some(message) = receiver.recv().await {
        println!("Received message: {}", message);
    }
}
