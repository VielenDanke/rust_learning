/// could be 3 types: custom-derive, attribute-like, function-like
/// has to be located in their own crate of specific type
// use proc_macro;
//
// #[some_attributes]
// pub fn some_name(input: TokenStream) -> TokenStream {}

pub trait HelloMacro {
    fn hello_macro();
}

/// Function approach
struct Pancakes;

impl HelloMacro for Pancakes {
    fn hello_macro() {
        println!("Hello, Macro! My name is Pancakes!");
    }
}

pub fn derive_macros_example() {
    Pancakes::hello_macro();
}
