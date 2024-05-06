pub mod basics;
pub mod async_primitives;
pub mod channels;

pub async fn tokio_spawn_example() {
    basics::tokio_spawn::spawn_async_task().await;
}

pub async fn tokio_spawn_blocking_example() {
    basics::tokio_spawn::spawn_blocking_task().await;
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_add_numbers() {
        assert_eq!(2, basics::tokio_spawn::add_numbers(1, 1).await);
    }
}
