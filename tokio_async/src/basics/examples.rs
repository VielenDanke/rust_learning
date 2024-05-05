use std::{thread};
use tokio::time::{sleep};

fn blocking_call() -> String {
    thread::sleep(std::time::Duration::from_secs(5));

    "Finally done".to_string()
}

async fn async_call(id: i32) {
    sleep(tokio::time::Duration::from_secs(1)).await;
    println!("Async Call: {}", id);
}

pub async fn example() {
    let blocking_handle = tokio::task::spawn_blocking(|| {
        return blocking_call();
    });

    let mut async_handles = Vec::new();

    for id in 0..10 {
        async_handles.push(
            tokio::spawn(async_call(id))
        );
    }
    for handle in async_handles {
        handle.await.unwrap();
    }
    let result = blocking_handle.await.unwrap();

    println!("Blocking call result: {}", result);
}
