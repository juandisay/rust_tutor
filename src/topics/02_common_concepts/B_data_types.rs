// Rust has two main data type subsets: scalar and compound.

fn main() {
    // SCALAR TYPES
    // Represent a single value.

    // Integer Types (e.g., i8, u8, i32, u32, i64, u64, isize, usize)
    let a: i32 = -10; // Signed 32-bit integer
    let b: u32 = 10;  // Unsigned 32-bit integer

    // Floating-Point Types (f32, f64)
    let c: f64 = 2.0; // 64-bit float (default)
    let d: f32 = 3.0; // 32-bit float

    // Boolean Type (bool)
    let e: bool = true;

    // Character Type (char) - represents a single Unicode Scalar Value
    let f: char = '😻';

    println!("Scalar Types:");
    println!("Integer: {}, Unsigned: {}", a, b);
    println!("Float 64: {}, Float 32: {}", c, d);
    println!("Boolean: {}", e);
    println!("Char: {}", f);


    // COMPOUND TYPES
    // Group multiple values into one type.

    // Tuple Type - a fixed-size collection of values of different types.
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    println!("
Compound Types:");
    println!("Tuple values from destructuring: x={}, y={}, z={}", x, y, z);
    println!("Accessing tuple directly: First value is {}", tup.0);


    // Array Type - a fixed-size collection of values of the same type.
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Array's first element: {}", arr[0]);
}
