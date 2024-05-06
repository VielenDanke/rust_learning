use std::sync::Arc;

use tokio::sync::{Barrier, BarrierWaitResult};

async fn barrier_work(barrier: Arc<Barrier>) -> BarrierWaitResult {
    println!("Waiting at the barrier");

    let wait_result = barrier.wait().await;

    println!("Passed through the barrier");

    wait_result
}

pub async fn barrier_example() {
    let total_cans_needed = 12;

    let barrier = Arc::new(Barrier::new(total_cans_needed));

    let mut task_handles = Vec::new();

    for _ in 0..60 {
        task_handles.push(
            tokio::spawn(barrier_work(barrier.clone()))
        );
    }
    let mut num_of_leaders = 0;

    for handle in task_handles {
        let wait_result = handle.await.unwrap();
        if wait_result.is_leader() {
            num_of_leaders += 1;
        }
    }
    println!("The number of leaders is {}", num_of_leaders);
}

