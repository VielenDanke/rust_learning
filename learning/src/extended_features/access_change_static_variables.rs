static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

pub fn add_to_static() {
    add_to_count(3);

    unsafe {
        println!("COUNTER: {COUNTER}");
    }
}
