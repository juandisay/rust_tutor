// Rust groups errors into two major categories: recoverable and unrecoverable errors.
// For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation.
// Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

use std::fs::File;
use std::io::{self, Read};

fn main() {
    // UNRECOVERABLE ERRORS WITH `panic!`
    // The `panic!` macro stops execution.
    // A backtrace will show the call stack that led to the panic.
    // Uncomment the line below to see a panic.
    // panic!("farewell, cruel world");


    // RECOVERABLE ERRORS WITH `Result`
    // The `Result` enum is defined as having two variants, `Ok` and `Err`:
    // enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        }
    };

    // The `unwrap` and `expect` methods are shortcuts for this `match` expression.
    // `unwrap` will call `panic!` on an `Err` variant.
    // `expect` is similar but lets you choose the `panic!` message.
    // let f_unwrap = File::open("hello.txt").unwrap();
    // let f_expect = File::open("hello.txt").expect("Failed to open hello.txt");


    // PROPAGATING ERRORS
    // When you’re writing a function whose implementation might fail, you can return
    // the error to the calling code.
    match read_username_from_file() {
        Ok(s) => println!("Read username: {}", s),
        Err(e) => println!("Error reading username: {:?}", e),
    }
}

// This function returns a `Result<String, io::Error>`.
// If the operations succeed, it returns an `Ok` containing the username.
// If any operation fails, it returns an `Err` containing the `io::Error`.
fn read_username_from_file() -> Result<String, io::Error> {
    // The `?` operator is a shortcut for propagating errors.
    // If the value of the `Result` is an `Ok`, the value inside the `Ok` will get returned from this expression.
    // If the value is an `Err`, the `Err` will be returned from the whole function as if we had used the `return` keyword.
    let mut f = File::open("username.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)

    // This can be chained for even shorter code:
    // File::open("username.txt")?.read_to_string(&mut s)?;
}
