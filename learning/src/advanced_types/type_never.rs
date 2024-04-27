// diverging functions
// using diverging function to return from None type Never to satisfy compiler
// pub fn bar() -> ! {}
// pub fn example_match(x: Option<u32>) -> u32 {
//     match x {
//         None => bar(),
//         Some(x) => x,
//     }
// }
