use std::fmt::Display;

pub struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// the methods will appear only if T implements both Display and PartialOrd
impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("X is greater than Y");
        } else {
            println!("Y is greater than X");
        }
    }
}

