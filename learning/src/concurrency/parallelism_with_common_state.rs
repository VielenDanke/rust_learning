use std::sync::{Arc, Mutex};
use std::thread;

pub fn mutex_example() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap(); // MutexGuard<i32> is the received type
        *num = 6;
        // by the end of this block the MutexGuard<i32> will be dropped and lock will be released
    }
    println!("m = {m:?}");
}

// Arc<T> - analogue Rc<T> (multiple immutable links), but for concurrent environment
pub fn mutex_multithreading_example() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}
