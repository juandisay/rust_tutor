fn main() {
    // By default, variables in Rust are immutable.
    let x = 5;
    println!("The value of x is: {}", x);

    // To make a variable mutable, we use the `mut` keyword.
    let mut y = 10;
    println!("The initial value of y is: {}", y);
    y = 20;
    println!("The new value of y is: {}", y);

    // Rust can also infer types.
    let z = 3.14; // f64 by default
    let is_rust_fun = true; // bool

    println!("The value of z is: {}", z);
    println!("Is Rust fun? {}", is_rust_fun);

    // You can declare constants using the `const` keyword.
    // Constants must be type-annotated and can be declared in any scope, including the global scope.
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
}
