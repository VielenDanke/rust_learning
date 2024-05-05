use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::time::{sleep, Duration};

async fn person(remote_arc: Arc<Mutex<i32>>, name: String, new_channel: i32) {
    *remote_arc.lock().await = new_channel;
    println!("Channel is changed to {} by {}", new_channel, name);
    sleep(Duration::from_secs(5)).await;
}

pub async fn mutex_example() {
    let remote_arc = Arc::new(Mutex::new(15));

    let names = vec!["Andrew".to_string(), "John".to_string(), "Josh".to_string(), "Peter".to_string()];

    let mut handles = Vec::new();

    for (i, name) in names.iter().enumerate() {
        let handle = tokio::spawn(person(remote_arc.clone(), name.clone(), i as i32));
        handles.push(handle);
    }
    for handle in handles {
        match handle.await {
            Ok(_) => {},
            Err(err) => {
                eprintln!("Join error: {err:?}");
            }
        }
    }
}
