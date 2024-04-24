pub fn raw_pointers() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is {}", *r2);
    }
}

pub fn raw_pointer_with_unknown_value_in_memory() {
    let address = 0x012345usize;
    let r = address as *const i32;
}
