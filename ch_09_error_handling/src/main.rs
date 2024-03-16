#![allow(unused_variables)]
use std::fs::File;
use std::io::{Error, ErrorKind};

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

fn main() {
    matching_different_errors();
    matching_different_errors_using_closure();
}
