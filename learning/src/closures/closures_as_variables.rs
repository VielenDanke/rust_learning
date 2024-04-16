use std::thread;
use std::time::Duration;

fn example_closure_with_variable() {
    let expensive_closure = |num: u32| -> u32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
}

fn example_with_and_without_type_annotation() {
    fn add_one_v1(x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 };
    let add_one_v4 = |x| x + 1;

    add_one_v4(15usize);
    // add_one_v4(15i32); won't compile because the compiler defined add_one_v4 function to use usize
}
