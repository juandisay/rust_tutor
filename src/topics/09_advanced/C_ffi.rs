// Foreign Function Interface (FFI) is a way for a programming language to define functions
// and enable a different (foreign) programming language to call those functions.


// SECTION: Calling a C function from Rust

// The `extern` keyword makes it possible to create and use a Foreign Function Interface (FFI).
// The `"C"` part defines which application binary interface (ABI) the external function uses.
extern "C" {
    // This is a function signature from the C standard library.
    fn abs(input: i32) -> i32;
}


// SECTION: Exposing a Rust function to C

// We can also use `extern` to create an interface that allows other languages to call Rust functions.
// The `#[no_mangle]` attribute is used to tell the Rust compiler not to mangle the name of this function.
#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Just called a Rust function from C!");
}

// To use this, you would:
// 1. Compile the Rust code into a static or dynamic library.
// 2. Link that library to your C code.
// 3. Include a header file in your C code with the function signature:
//    `void call_from_c(void);`


fn main() {
    // To call the C function, we must use an `unsafe` block.
    // The `unsafe` keyword gives us access to five superpowers in Rust that aren't
    // normally available, including calling foreign functions.
    // This is because Rust can't guarantee memory safety for the external code.
    println!("--- Calling C from Rust ---");
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    println!("\n--- Exposing Rust to C ---");
    println!("This file also defines a function `call_from_c` that can be called from C code.");
    println!("See the comments in the source for how to do this.");
}

// To make the `abs` example runnable, you would need to link against the C standard library.
// For many systems, this happens by default.
