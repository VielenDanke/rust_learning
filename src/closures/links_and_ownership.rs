use std::thread;

pub fn borrows_immutable_reference() {
    let list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

pub fn borrows_mutable_reference() {
    let mut list = vec![1, 2, 3];

    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

// we're using move to change ownership of the variable `list`
pub fn ownership() {
    let list = vec![1, 2, 3];

    println!("Before defining closure");

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

// 3 types of closures
// FnOnce() -> T - the function is called only once, returns a value from it's body, do not accept any values
// FnMut(V) -> T - could be called more than once, could modify state, do not return anything from it's body
// Fn(V) - could be called more than once, couldn't modify state, could be used without any capture at all
//      do not return anything from it's body
/*
impl<T> Option<T> {
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T
    {
        match self {
            Some(x) => x,
            None => f(),
        }
    }
}
 */
