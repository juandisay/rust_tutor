// Functions are declared with the `fn` keyword.
// Rust code uses snake_case as the conventional style for function and variable names.

fn main() {
    println!("Hello from main!");
    another_function(5, 'h');

    // Statements and Expressions
    // Statements are instructions that perform some action and do not return a value.
    // `let x = 6;` is a statement.
    // Expressions evaluate to a resultant value.
    let y = {
        let x = 3;
        x + 1 // No semicolon at the end makes it an expression
    };
    println!("The value of y from the expression is: {}", y);

    let five = five();
    println!("The value from the five function is: {}", five);

    let plus_one_result = plus_one(five);
    println!("The result of plus_one is: {}", plus_one_result);
}

// A function with parameters. Type annotations are required.
fn another_function(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}

// A function with a return value.
// The return type is specified after an arrow `->`.
// The return value is the value of the final expression in the function body.
fn five() -> i32 {
    5 // Return expression
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
