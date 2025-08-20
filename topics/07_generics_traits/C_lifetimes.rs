// Lifetimes are another kind of generic that ensure that references are valid as long as we need them to be.
// Every reference in Rust has a lifetime, which is the scope for which that reference is valid.

// The main aim of lifetimes is to prevent dangling references.
fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // Example of a dangling reference that the compiler will prevent:
    // let r;
    // {
    //     let x = 5;
    //     r = &x;
    // } // `x` goes out of scope here
    // println!("r: {}", r); // `r` would be a dangling reference


    // LIFETIMES IN STRUCTS
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
    println!("Important excerpt: {}", i.part);
}

// The generic lifetime parameter `'a` defines a relationship between the lifetimes of the references.
// This function signature tells Rust that for some lifetime `'a`, the function takes two parameters,
// both of which are string slices that live at least as long as lifetime `'a`.
// The function will return a string slice that also lives at least as long as lifetime `'a`.
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Structs can also hold references, but they need a lifetime annotation.
struct ImportantExcerpt<'a> {
    part: &'a str,
}
