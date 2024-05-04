pub mod example;
pub mod closures_as_variables;
pub mod links_and_ownership;

// 3 types of closures
// FnOnce() -> T - the function is called only once, returns a value from it's body, do not accept any values
// FnMut(V) -> T - could be called more than once, could modify state
// Fn(V) -> T - could be called more than once, couldn't modify state
