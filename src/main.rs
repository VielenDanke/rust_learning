pub mod learning;

fn main() {
    learning::exceptions::recoverable_result::file_not_exist_recoverable(&"hello.txt".to_string())
}
