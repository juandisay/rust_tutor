// A slice lets you reference a contiguous sequence of elements in a collection
// rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

fn main() {
    let mut s = String::from("hello world");

    let word_index = first_word(&s);

    println!("The first word ends at index: {}", word_index);

    // STRING SLICES
    // A string slice is a reference to part of a String.
    let hello = &s[0..5]; // or &s[..5]
    let world = &s[6..11]; // or &s[6..]

    println!("Slices: {} {}", hello, world);

    // A better `first_word` using slices
    let word = first_word_slice(&s);

    // s.clear(); // This would cause a compile error because `word` is an immutable borrow.

    println!("The first word is: {}", word);


    // OTHER SLICES
    // String slices are specific to strings. We can also have general slices.
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // This slice has the type &[i32]

    assert_eq!(slice, &[2, 3]);
    println!("The array slice is: {:?}", slice);
}

// This function takes a reference to a String and returns the index of the end of the first word.
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

// This improved version returns a string slice.
// The type `&str` refers to a string slice.
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
