// Rust’s `String` type is a growable, mutable, owned, UTF-8 encoded string type.
// Rust also has another string type, the string slice `&str`.

fn main() {
    // Creating a new String
    let mut s = String::new();

    let data = "initial contents";
    let s = data.to_string();
    println!("s from .to_string(): {}", s);

    let s = String::from("initial contents");
    println!("s from String::from(): {}", s);

    // Updating a String
    let mut s = String::from("foo");
    s.push_str("bar");
    println!("s after push_str: {}", s);

    s.push('l');
    println!("s after push: {}", s);

    // Concatenation with the `+` Operator or the `format!` Macro
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("Concatenated string: {}", s3);

    let t1 = String::from("tic");
    let t2 = String::from("tac");
    let t3 = String::from("toe");
    let t = format!("{}-{}-{}", t1, t2, t3);
    println!("Formatted string: {}", t);


    // Indexing into Strings
    // Rust strings do not support indexing like `s[0]` because UTF-8 characters
    // can have variable byte lengths.

    // Slicing Strings
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // This will be "Зд"
    println!("Sliced string: {}", s);

    // Iterating over Strings
    println!("Iterating over chars:");
    for c in "नमस्ते".chars() {
        println!("  {}", c);
    }

    println!("Iterating over bytes:");
    for b in "नमस्ते".bytes() {
        println!("  {}", b);
    }
}
