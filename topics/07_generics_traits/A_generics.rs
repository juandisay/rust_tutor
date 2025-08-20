// Generics are a tool for creating definitions for items like function signatures or structs,
// which we can then use with many different concrete data types.

// A generic function `largest` that can find the largest element in a slice of any type `T`.
// The `PartialOrd` and `Copy` traits are required for type `T`.
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Generic Structs
struct Point<T, U> {
    x: T,
    y: U,
}

// Generic Methods
impl<T, U> Point<T, U> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let both_integer = Point { x: 5, y: 10 };
    let both_float = Point { x: 1.0, y: 4.0 };
    let integer_and_float = Point { x: 5, y: 4.0 };

    println!("both_integer.x = {}", both_integer.x());
    println!("integer_and_float.y = {}", integer_and_float.y);
}
