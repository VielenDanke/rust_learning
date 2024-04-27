use std::rc::Rc;

/// dynamic types in Rust contain extra bit of metadata which stores the size of dynamic info
/// We need to store dynamic types always behind some pointer
pub fn example() {
    let dynamic_s1: &str = "str_1";
    let dynamic_s2 = Rc::<str>::from("foo");
}

/// ? declines the rule that size of the T types should be known during compile time
/// The ? is working only with Sized trait
pub fn generic<T: ?Sized>(t: &T) {

}
