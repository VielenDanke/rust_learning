use std::fs;
use std::sync::Arc;

use tokio::sync::RwLock;

async fn read_from_file(rw_file_lock: Arc<RwLock<String>>) {
    let read_guard = rw_file_lock.read().await;

    let buff = fs::read_to_string(read_guard.as_str()).unwrap();

    println!("Read from file {buff}");
}

async fn write_to_file(rw_file_lock: Arc<RwLock<String>>, message: String) {
    let write_guard = rw_file_lock.write().await;

    fs::write(write_guard.as_str(), message.as_bytes()).unwrap();

    println!("Write to file: {message}");
}

pub async fn rw_example() {
    let rw_file_lock = Arc::new(RwLock::new(String::from("rw_file_example.log")));

    let mut handles = Vec::new();

    for i in 0..100 {
        if i % 5 == 0 {
            handles.push(
                tokio::spawn(write_to_file(rw_file_lock.clone(), format!("Message {}", i).to_string()))
            );
        } else {
            handles.push(
                tokio::spawn(read_from_file(rw_file_lock.clone()))
            );
        }
    }

    for handle in handles {
        match handle.await {
            Ok(_) => {},
            Err(err) => eprintln!("Join Error: {err:?}"),
        }
    }
}
