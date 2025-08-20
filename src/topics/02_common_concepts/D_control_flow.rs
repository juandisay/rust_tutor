fn main() {
    // `if` expressions
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // Using `if` in a `let` statement
    let condition = true;
    let x = if condition { 5 } else { 6 };
    println!("The value of x is: {}", x);


    // Repetition with Loops
    // Rust has three kinds of loops: `loop`, `while`, and `for`.

    // `loop`: Executes a block of code forever or until you explicitly tell it to stop.
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // `break` can return a value from the loop
        }
    };
    println!("The result from the loop is {}", result);


    // `while`: A conditional loop.
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");


    // `for`: Loop over a collection.
    let a = [10, 20, 30, 40, 50];
    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // `for` with a Range
    for number in (1..4).rev() { // 1, 2, 3 reversed
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
