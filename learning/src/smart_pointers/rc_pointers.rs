use std::rc::Rc;
use crate::smart_pointers::recursive_types_with_box::ListBox::{Cons, Nil};

pub fn box_example_non_compiled() {
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a)); not possible, wont compile because value a moved before, and here is used after it was moved
}

enum ListRc<T> {
    Cons(T, Rc<ListRc<T>>),
    Nil,
}

pub fn rc_example() {
    let a = Rc::new(ListRc::Cons(5, Rc::new(ListRc::Cons(10, Rc::new(ListRc::Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = ListRc::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = ListRc::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
