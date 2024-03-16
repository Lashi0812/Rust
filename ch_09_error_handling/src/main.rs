#![allow(unused_variables, dead_code)]

use std::fs::{self, File};
use std::io::{Error, ErrorKind, Read};

fn matching_different_errors() {
    let file_result: Result<File, Error> = File::open("hello.txt");
    let file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(error) => panic!("Problem creating the file : {error:?}"),
            },
            other_error => {
                panic!("Error has caused due to {other_error:?}");
            }
        },
    };
}

fn matching_different_errors_using_closure() {
    let file = File::open("hello_closure.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello_closure.txt").unwrap_or_else(|error| {
                panic!("Problem while creating the file {error:?}");
            })
        } else {
            panic!("Problem while opening the file {error:?}")
        }
    });
}

fn matching_different_errors_using_unwarp() {
    /*
        calling the unwarp on Result ,
            if its Ok then then return the file
            if its Err then call the panic macro
    */
    let file = File::open("using_unwarp").unwrap();
}

fn matching_different_errors_using_except() {
    /*
        it is similar to unwarp but we can specify the error message
    */
    let file = File::open("using_except.txt").expect("using_except.txt not found");
}

fn long_way_read_username_from_file() -> Result<String, Error> {
    let user_file_result: Result<File, Error> = File::open("hello.txt");
    let mut username_file = match user_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn error_propagate_operator() -> Result<String, Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn chaining_error_propagate_operator() -> Result<String, Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn using_builtin_fn() -> Result<String, Error> {
    fs::read_to_string("hello.txt")
}

fn propagating_error() {
    // let username = long_way_read_username_from_file().expect("🚨🚨Cannot find the file");
    // let username = error_propagate_operator().expect("🚨🚨Cannot find the file");
    let username = chaining_error_propagate_operator().expect("🚨🚨Cannot find the file");
}

fn main() {
    // matching_different_errors();
    // matching_different_errors_using_closure();
    // matching_different_errors_using_unwarp();
    // matching_different_errors_using_except();
    propagating_error();
}
