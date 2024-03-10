pub mod collections;
pub mod exceptions;

fn main() {
    exceptions::recoverable_result::file_not_exist_recoverable(&"hello.txt".to_string())
}
