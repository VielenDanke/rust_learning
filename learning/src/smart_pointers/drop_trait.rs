#[derive(Debug)]
struct CustomSmartPointer {
    data: String,
}

impl CustomSmartPointer {
    fn new(str: String) -> Self {
        CustomSmartPointer { data: str }
    }
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

pub fn example() {
    let pointer1 = CustomSmartPointer::new("my stuff".to_string());
    let pointer2 = CustomSmartPointer::new("other stuff".to_string());
    println!("Custom smart pointers created: {:?} and {:?}", pointer1, pointer2);
}

pub fn example_std_mem_drop() {
    let c = CustomSmartPointer::new("some data".to_string());
    println!("CustomSmartPointer created.");
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
}
