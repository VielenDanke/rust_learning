extern "C" {
    fn abs(input: i32) -> i32;
}

pub fn call() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}

#[no_mangle] // do not change the function name
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}
