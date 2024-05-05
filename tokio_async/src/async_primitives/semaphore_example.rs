use std::sync::Arc;

use tokio::sync::Semaphore;
use tokio::time::{Duration, sleep};

async fn person(semaphore_arc: Arc<Semaphore>, name: String) {
    println!("{} is waiting in line", &name);
    teller(semaphore_arc, name).await
}

async fn teller(semaphore_arc: Arc<Semaphore>, customer: String) {
    let permit = semaphore_arc.acquire().await.unwrap();

    sleep(Duration::from_secs(2)).await;

    println!("{} is being server by the teller", &customer);

    sleep(Duration::from_secs(5)).await;

    println!("{} is now leaving the teller", &customer);

    // permit will be automatically dropped at the end of the function
    // write the drop function explicitly
    drop(permit);
}

pub async fn semaphore_example() {
    let num_of_tellers = 4;

    let sem = Semaphore::new(num_of_tellers);
    let sem_arc = Arc::new(sem);

    let names = (0..10).into_iter().map(|v| format!("{} name", v).to_string()).collect::<Vec<String>>();

    let mut handles = Vec::new();

    for name in names {
        let handle = tokio::spawn(person(sem_arc.clone(), name));
        handles.push(handle);
    }
    for handle in handles {
        if let Err(err) = handle.await {
            eprintln!("Join Error: {err:?}");
        }
    }
}
