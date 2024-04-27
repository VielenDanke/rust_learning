/// to return closures we need to wrap it in a wrapper struct
/// because the compiler doesn't know the size to allocate for the closure
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
