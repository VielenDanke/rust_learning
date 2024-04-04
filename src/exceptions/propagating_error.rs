use std::fs::{File, read_to_string};
use std::io;
use std::io::Read;

pub fn read_username_from_file(file_name: &String) -> Result<String, io::Error> {
    let username_file_result = File::open(file_name);

    let mut username_file = match username_file_result {
        Ok(fs) => fs,
        Err(e) => return Err(e),
    };
    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e)
    }
}

pub fn read_username_from_file_shorter(file_name: &String) -> Result<String, io::Error> {
    // if the operation succeed we create new variable called username_file (return Ok(fs))
    // if the operation fails we return error as we do in read_username_from_file
    // the error transforms to the type declared in return statement in the function
    let mut username_file = File::open(file_name)?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

pub fn read_username_from_file_even_shorter(file_name: &String) -> Result<String, io::Error> {
    let mut username = String::new();

    File::open(file_name)?.read_to_string(&mut username)?;

    Ok(username)
}

pub fn read_username_from_file_library_function(file_name: &String) -> Result<String, io::Error> {
    read_to_string(file_name)
}

pub fn use_question_mark_with_option(text: &str) -> Option<char> {
    // in case of next return Option<Some> the functions continues
    // in case of next return Option<None> the functions stops and returns Option<None> to the caller
    text.lines().next()?.chars().last()
}
