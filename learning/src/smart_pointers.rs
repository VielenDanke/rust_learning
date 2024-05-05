/*
Here is a list of reasons for choosing the types Box<T>, Rc<T> or RefCell<T>:

The Rc<T> type allows multiple ownership of the same data; the Box<T> and RefCell<T> types allow for single owners.
The Box<T> type allows immutable or mutable properties that are checked during compilation;
the Rc<T> type only allows immutable properties that are checked during compilation;
the RefCell type<T> allows immutable or mutable properties checked at runtime.
Since RefCell<T> allows mutable borrowings checked at runtime, it is possible to change the value inside RefCell<T> even if RefCell<T> is immutable.
 */
pub mod box_example;
pub mod recursive_types_with_box;
pub mod box_as_link;
pub mod drop_trait;
pub mod rc_pointers;
pub mod refcell;
pub mod rc_with_refcell;
