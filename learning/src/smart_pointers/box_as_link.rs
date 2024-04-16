pub fn hello(name: &str) {
    println!("Hello, {name}!");
}

pub fn call_hello() {
    let name = Box::new(String::from("Rust"));
    hello(&name);
}
