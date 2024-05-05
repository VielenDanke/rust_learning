#[tokio::main]
async fn main() {
    // tokio_spawn_example().await;
    // tokio_spawn_blocking_example().await;
    // tokio_async::async_primitives::mutex_example::mutex_example().await;
    // example().await;
    tokio_async::async_primitives::semaphore_example::semaphore_example().await;
}


