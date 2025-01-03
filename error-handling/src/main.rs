// Error handling in Rust:
// Rust groups errors into two major categroies: recoverable and unrecoverable errors.
// Recoverable errors are those that a program can reasonably be expected to recover from.
// Unrecoverable errors are those that a program cannot recover from.
// Rust does not have exceptions. Instead, it uses the Result enum to handle errors. Result<T, E>
// panic! macro is used to stop the execution of a program when an unrecoverable error occurs.

// By default when a panic occurs, the program starts unwinding, which means Rust walks back up the stack and cleans up the data from each function it encounters.
// However, this is a lot of work, and it has a runtime cost. Alternatively, you can set the panic to immediately abort the program, which is faster, but doesn't run cleanup code.
// You can set this behavior by adding the following to your Cargo.toml file:
// [profile.release]
// panic = 'abort'

use std::fs;
use std::fs::File;
use std::error::Error;
// Matching on Different Errors
use std::io::{self, ErrorKind, Read};

// For nowm Box<dyn Error> is a stand-in for "any kind of error".

// The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode. 

fn main() -> Result<(), Box<dyn Error>> {
    // panic!("crash and burn");

    let v = vec![1, 2, 3];

    // v[99]; // This wil cause an "out of bounds" panic error.

    // Recoverable errors and Result enum

    // The below code result type is Result<T, E> where T is the type of the value that will be returned in a success case within the Ok variant, and E is the type of the error that will be returned in a failure case within the Err variant.
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            other_error => {
                panic!("Problem opening the file: {other_error:?}");
            }
        },
    };

    // Shortcuts for Panic on Error: unwrap and expect
    // The unwrap method is a shortcut method that is implemented on Result types.
    // If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic! macro for us.
    let greeting_file = File::open("hello.txt").unwrap();

    // The expect method is similar to unwrap, but it allows us to specify the panic! error message.
    let greeting_file = File::open("hello.txt").expect("Failed to open hello.txt");

    Ok(())
}

// Propagating Errors
// When you're writing a function whose implementation calls some other function that might fail, you have two choices: you can either return the error to the calling code or you can call panic! and have the program crash if this function fails.
// The ? operator can only be used in functions that have a return type of Result, because it is defined to work in the same way as the match expression.
// If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue.
// If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.
// Below is the long version of the read_username_from_file function without using the ? operator.
fn read_username_from_file_match() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// The ? operator can be used in the read_username_from_file function to shorten the code.
// Basically the ? operator explicitly propagates errors.
fn read_username_file() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Chain multiple ? operators to shorten the code even further.
fn read_username_file_short() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}

fn read_username_file_shorter() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
} // This is the shortest version of the read_username_file function.


// The ? operator can also be used with the Option enum.
// If the Option is a Some value, the value inside the Some will get returned from the expression.
// If the Option is a None value, the None will be returned from the whole function as if we had used the return keyword.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}