use std::fs::File;
use std::io::ErrorKind;
/*
enum Result<T, E> {
    Ok(T), // T - success result variable
    Err(E), // E - error result variable
}
 */

pub fn file_not_exist_recoverable(file_name: &String) {
    let result = File::open(file_name);
    match result {
        Ok(f) => println!("File exist - {:?}", f.metadata().unwrap()),
        Err(err) => println!("File doesn't exist - {err}"),
    }
}

pub fn file_not_exist_recoverable_different_types(file_name: &String) -> File {
    match File::open(file_name) {
        Ok(file) => file,
        Err(err) => {
            match err.kind() {
                ErrorKind::NotFound => match File::create(file_name) {
                    Ok(fc) => fc,
                    Err(err) => panic!("Problem creating the file: {:?}", err),
                },
                other_error => panic!("Problem opening the file: {:?}", other_error),
            }
        }
    }
}

pub fn file_not_exist_recoverable_different_types_alternative(file_name: &String) -> File {
    File::open(file_name).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(file_name).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
}

