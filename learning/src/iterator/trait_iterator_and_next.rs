/*
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}

.iter() - immutable links
.iter_mut() - mutable links
.into_iter() - borrow iterator
 */



pub fn create_iterator() {
    let list = vec![1, 2, 3];

    let list_iter = list.iter();

    for val in list_iter {
        println!("Got: {}", val);
    }
}
