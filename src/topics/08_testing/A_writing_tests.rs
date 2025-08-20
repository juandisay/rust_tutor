// Rust has first-class support for writing and running tests.
// Tests are Rust functions that verify that the non-test code is functioning in the expected manner.

// The bodies of test functions typically perform these three actions:
// 1. Set up any needed data or state.
// 2. Run the code you want to test.
// 3. Assert that the results are what you expect.

// To run the tests in this file, use the command `rustc --test <filename>`
// This will create and run a test executable.


// SECTION: DOCUMENTATION TESTS
// Rust can run tests found within your documentation comments.
// This is great for ensuring your examples are always correct.

/// Adds two to the given number.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = rust_tutor::add_two(arg);
///
/// assert_eq!(7, answer);
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}


// SECTION: UNIT TESTS
// The `#[cfg(test)]` annotation tells Rust to compile and run the test code only when you run `cargo test`,
// not when you run `cargo build`.
#[cfg(test)]
mod tests {
    // We need to bring the code we want to test into scope.
    use super::*;

    // The `#[test]` attribute indicates this is a test function.
    #[test]
    fn it_adds_two() {
        // `assert_eq!` macro checks for equality.
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn another() {
        // `assert!` macro checks if the expression is true.
        assert!(add_two(10) == 12, "10 plus 2 should be 12");
    }

    // Testing with `should_panic`
    // This test passes if the code inside the function panics.
    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be greater than or equal to 1, got {}.",
                   value);
        } else if value > 100 {
            panic!("Guess value must be less than or equal to 100, got {}.",
                   value);
        }

        Guess { value }
    }
}


// SECTION: INTEGRATION TESTS
//
// Integration tests are external to your library. They are meant to test your code's public API.
// To create an integration test, you would typically do the following:
//
// 1. Create a `tests` directory next to your `src` directory.
// 2. Inside `tests`, create a new Rust file (e.g., `tests/integration_test.rs`).
// 3. In that file, you would use `use your_crate_name;` to import your library.
// 4. Write test functions just like in unit tests, but without the `#[cfg(test)]` module.
//
// Example of what `tests/integration_test.rs` might contain:
//
// use rust_tutor; // Assuming your crate is named `rust_tutor` in Cargo.toml
//
// #[test]
// fn test_add_two_integration() {
//     assert_eq!(4, rust_tutor::add_two(2));
// }
//
// You would run these tests with `cargo test`.
// Cargo knows to look for integration tests in the `tests` directory.
