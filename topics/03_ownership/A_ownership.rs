// Ownership is Rust's most unique feature.
// It enables Rust to make memory safety guarantees without needing a garbage collector.

fn main() {
    // VARIABLE SCOPE
    // A scope is the range within a program for which an item is valid.
    {
        let s = "hello"; // s is valid from this point forward
        // do stuff with s
        println!("s is: {}", s);
    } // this scope is now over, and s is no longer valid


    // THE `String` TYPE
    // A type that manages data allocated on the heap.
    let mut s = String::from("hello"); // `s` is valid from this point forward.
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);
    // The memory is automatically returned once the variable that owns it goes out of scope.


    // MOVE
    // When we assign a heap-allocated value (like a String) to another variable,
    // the ownership is moved.
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2. s1 is no longer valid.

    // println!("s1 is {}, s2 is {}", s1, s2); // This line would cause a compile error!
    println!("s2 is {}", s2);


    // CLONE
    // If we want to deeply copy the heap data, not just move ownership, we can use `clone()`.
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);


    // OWNERSHIP AND FUNCTIONS
    let s = String::from("hello");  // s comes into scope
    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope
    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward
    println!("x is still valid: {}", x);

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("Inside takes_ownership: {}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("Inside makes_copy: {}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
