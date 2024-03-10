use std::fs::File;

pub fn open_file_unwrap(file_name: &String) -> File {
    File::open(file_name).unwrap()
}

pub fn open_file_expect(file_name: &String) -> File {
    File::open(file_name)
        .expect("This is a message for panic in case of error opening the file")
}
