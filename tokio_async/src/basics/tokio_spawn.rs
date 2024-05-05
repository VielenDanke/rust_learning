async fn hello(name: &str) -> String {
    let current_thread = std::thread::current();
    let n = current_thread.name().unwrap_or("name is unknown");
    format!("Hello, {name} from thread: {n}")
}

fn hello_blocking(name: &str) -> String {
    let current_thread = std::thread::current();
    let n = current_thread.name().unwrap_or("name is unknown");
    format!("Blocking Hello, {name} from thread: {n}")
}

pub async fn add_numbers(a: i32, b: i32) -> i32 {
    a + b
}

// provide a function call on a tokio worker
// Applications should feel free to spawn thousands, if not millions of tasks
pub async fn spawn_async_task() {
    let join_handle = tokio::spawn(hello("Name"));

    match join_handle.await {
        Ok(val) => println!("{val}"),
        Err(err) => println!("Error occurred {err:?}"),
    }
}

// provide a function call on a thread dedicated to blocking operation (in our case - main thread)
// https://dtantsur.github.io/rust-openstack/tokio/task/fn.spawn_blocking.html
pub async fn spawn_blocking_task() {
    let handle = tokio::task::spawn_blocking(|| {
        return hello_blocking("Blocking Name");
    });
    match handle.await {
        Ok(val) => {
            println!("{val}");
        }
        Err(err) => println!("Error occurred: {err:?}"),
    }
}
