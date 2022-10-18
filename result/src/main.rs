extern crate core;

use std::error::Error;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    println!("Hello, world!");
    attempt_to_open_file_panic();
}
// fn main() -> Result<(), Box<dyn Error>> {
//     let greeting_file = File::open("hello.txt")?;
//
//     Ok(())
// }

fn attempt_to_open_file_panic() -> Result<File, ()> {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => Ok(file),
        Err(error) => panic!("Problem opening file:{:?}", error),
    };
    /* same as
     let greeting_file = File::open("hello.txt").unwrap();
     To customise what is printed by panic
     let greeting_file = File::open("hello.txt").expect("Custom panic! message");
    */
    greeting_file
}

fn attempt_to_open_file_error() {
    let greeting_file_result = File::open("hello.txt");
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file:{:?}", e),
            },
            other_error => {
                panic!("Problem opening the file : {:?}", other_error);
            }
        },
    };
}

fn less_code_error_handling() -> Result<String, io::Error> {
    let mut username = String::new();
    // "?" either returns an Error or the value from our Result
    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)

    /*
    same as
    fs::read_to_string("hello.txt") // this does all the Result handling we did earlier.
     */
}

fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
