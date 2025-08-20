// Vectors allow you to store more than one value in a single data structure
// that puts all the values next to each other in memory.
// Vectors can only store values of the same type.

fn main() {
    // Creating a new, empty vector
    let mut v: Vec<i32> = Vec::new();

    // Creating a vector with values using the `vec!` macro
    let v2 = vec![1, 2, 3];

    // Updating a vector
    v.push(5);
    v.push(6);
    v.push(7);

    println!("Vector v: {:?}", v);
    println!("Vector v2: {:?}", v2);

    // Reading elements of vectors
    let third: &i32 = &v2[2];
    println!("The third element of v2 is {}", third);

    // Using `get` to access an element, which returns an `Option<&T>`
    match v2.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }

    // Iterating over the values in a vector
    let mut v3 = vec![100, 32, 57];
    println!("Iterating over v3:");
    for i in &v3 {
        println!("  {}", i);
    }

    // Iterating over mutable references to elements in a vector
    println!("Iterating over mutable references in v3:");
    for i in &mut v3 {
        *i += 50; // Use the dereference operator (*) to get to the value
        println!("  {}", i);
    }

    // Using an enum to store multiple types in a vector
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    println!("Row with multiple types: {:?}", row);
}
