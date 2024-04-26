use std::fmt;
use std::fmt::Formatter;

struct VecWrapper(Vec<String>);

impl fmt::Display for VecWrapper {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}]", self.0.join(","))
    }
}

pub fn example() {
    let w = VecWrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {w}");
}
