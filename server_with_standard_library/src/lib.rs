use std::fmt::{Debug, Formatter};
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Result<Worker, std::io::Error> {
        let builder = thread::Builder::new();

        let thread = builder.spawn(move || loop {
            let message = receiver.lock().unwrap().recv();

            match message {
                Ok(job) => {
                    println!("Worker {id} got a job; executing.");

                    job();
                }
                Err(_) => {
                    println!("Worker {id} disconnected; shutting down");
                    break;
                }
            }
        })?;

        Ok(Worker { id, thread: Some(thread) })
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct PoolCreationError {
    msg: String,
}

impl Debug for PoolCreationError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.msg.as_str())
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Result<ThreadPool, PoolCreationError>
    ///
    /// The `build` function will return PoolCreationError if size is 0.
    /// The function will return PoolCreationError in case of Worker couldn't be created
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size == 0 {
            return Err(PoolCreationError { msg: String::from("Pool size has to be greater than 0") });
        }
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            let worker_result = Worker::new(id, Arc::clone(&receiver));
            if worker_result.is_err() {
                return Err(PoolCreationError { msg: format!("Cannot create pool worker: {:?}", worker_result.unwrap_err()) });
            }
            workers.push(worker_result.unwrap());
        }

        Ok(ThreadPool { workers, sender: Some(sender) })
    }

    /// Create a new ThreadPool with 1 thread.
    ///
    /// # Result<ThreadPool, PoolCreationError>
    ///
    /// The `new` function will return PoolCreationError it's not able to spawn a thread.
    pub fn new() -> ThreadPool {
        ThreadPool::build(1).unwrap()
    }

    /// Executes a function by sending it to the pool
    ///
    /// # Panic
    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

#[cfg(test)]
mod test {
    use std::ops::Deref;
    use std::sync::{Arc, Mutex};

    use super::*;

    #[test]
    fn worker_create() -> Result<(), String> {
        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let id = 0;

        let result = Worker::new(id, Arc::clone(&receiver));

        match result {
            Ok(worker) if id == worker.id => {
                Ok(())
            }
            Err(err) => {
                Err(err.to_string())
            }
            Ok(_) => {
                Err(format!("id from worker is not equal to {}", id))
            }
        }
    }

    #[test]
    fn thread_pool_create_with_threads() -> Result<(), String> {
        let result = ThreadPool::build(3);

        match result {
            Ok(_) => {
                Ok(())
            }
            Err(err) => {
                Err(err.msg)
            }
        }
    }

    #[test]
    fn thread_pool_create_default() {
        let result = ThreadPool::new();

        assert_eq!(1usize, result.workers.len());
    }

    #[test]
    fn thread_pool_executes_work() {
        // given
        let m = Arc::new(Mutex::new(0));

        fn test_exec(m: Arc<Mutex<i32>>) {
            *m.lock().unwrap() += 1;
        }

        let pool = ThreadPool::new();
        let to_execute_1 = Arc::clone(&m);
        let to_execute_2 = Arc::clone(&m);
        let to_execute_3 = Arc::clone(&m);

        // when
        pool.execute(|| {
            test_exec(to_execute_1);
        });
        pool.execute(|| {
            test_exec(to_execute_2);
        });
        pool.execute(|| {
            test_exec(to_execute_3);
        });
        drop(pool);

        // then
        assert_eq!(3, *m.lock().unwrap().deref());
    }
}
