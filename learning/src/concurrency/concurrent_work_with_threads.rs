use std::thread;
use std::time::Duration;

pub fn create_thread_with_spawn() {
    // create thread
    let handle = thread::spawn(|| {
       for i in 1..10 {
           println!("Hi number {i} from the spawned thread!");
           thread::sleep(Duration::from_millis(1));
       }
    });
    for i in 1..5 {
        println!("Hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}
