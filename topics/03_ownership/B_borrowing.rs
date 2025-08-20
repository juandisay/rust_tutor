// The issue with the `takes_ownership` example is that we have to return the String
// back to the calling function so we can still use it. This can be tedious.
// Rust has a feature for using a value without transferring ownership, called references.

fn main() {
    // REFERENCES AND BORROWING
    // A reference is like a pointer in that it’s an address we can follow to access data.
    // We call the action of creating a reference *borrowing*.
    let s1 = String::from("hello");

    // The `&s1` syntax lets us create a reference that *refers* to the value of `s1`
    // but does not own it. Because it does not own it, the value it points to
    // will not be dropped when the reference goes out of scope.
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);


    // MUTABLE REFERENCES
    // We can also borrow a mutable reference to modify a value.
    let mut s = String::from("hello");
    change(&mut s);
    println!("Modified string: {}", s);

    // A key rule of mutable references:
    // You can have only ONE mutable reference to a particular piece of data in a particular scope.
    // This prevents data races at compile time.
    let r1 = &mut s;
    // let r2 = &mut s; // This would be a compile error!
    // println!("{}, {}", r1, r2);

    // We also cannot have a mutable reference while we have an immutable one.
    let r_immut = &s;
    // let r_mut = &mut s; // COMPILE ERROR
    // println!("{}, {}", r_immut, r_mut);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, nothing happens.

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
