use std::fmt::{Debug, Display};
use crate::traits::create_and_implement_trait::Summary;

// restrict what we can use under T
pub fn notify<T: Summary<Item = String>>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

pub fn notify_with_restrictions(item: &(impl Summary + Display)) {}

pub fn notify_with_restrictions_generics<T: Summary + Display>(item: &T) {}

pub fn functions_with_complex_generics<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
    -1
}

pub fn functions_with_where<T, U>(t: &T, u: &U) -> i32
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    -1
}
