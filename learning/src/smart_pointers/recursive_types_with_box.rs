use crate::smart_pointers::recursive_types_with_box::ListBox::{Cons, Nil};

// #[derive(Debug)]
// enum List<T> {
//     Cons(T, List<T>),
//     Nil,
// }

#[derive(Debug)]
pub enum ListBox<T> {
    Cons(T, Box<ListBox<T>>),
    Nil,
}

// can't be compiled
// pub fn example_recursive_types() {
//     let list = Cons(1, Cons(2, Cons(3, List::Nil)));
// }

pub fn example_recursive_types_with_box() -> String {
    let list_box = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    format!("{:?}", list_box)
}
