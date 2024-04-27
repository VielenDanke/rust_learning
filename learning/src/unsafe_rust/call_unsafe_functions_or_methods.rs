use std::slice;

unsafe fn dangerous() {}

pub fn call_dangerous_function() {
    unsafe {
        dangerous();
        let ptr = &mut 5;
        *ptr += 1;
        drop(*ptr);
    }
}

pub fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
