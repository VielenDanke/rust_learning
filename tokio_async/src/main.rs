#[tokio::main]
async fn main() {
    // tokio_spawn_example().await;
    // tokio_spawn_blocking_example().await;
    // tokio_async::async_primitives::mutex_example::mutex_example().await;
    // example().await;
    // tokio_async::async_primitives::semaphore_example::semaphore_example().await;
    // tokio_async::async_primitives::notify_example::notify_example().await;
    // tokio_async::async_primitives::barrier_example::barrier_example().await;
    // tokio_async::async_primitives::rw_lock_example::rw_example().await;
    // tokio_async::channels::oneshot_channel::oneshot_example().await;
    // tokio_async::channels::watch_channel::watch_channel_example().await;
    // tokio_async::channels::mpsc_channel::mpsc_example().await;
    tokio_async::channels::broadcast_channel::broadcast_example().await;
}


