// A struct, or structure, is a custom data type that lets you package together
// and name multiple related values that make up a meaningful group.

// Defining a struct
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple Structs
// Tuple structs have the added meaning the struct name provides but don’t have names
// associated with their fields; rather, they just have the types of the fields.
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    // Instantiating a struct
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // Accessing struct fields using dot notation
    user1.email = String::from("anotheremail@example.com");
    println!("User email: {}", user1.email);

    let user2 = build_user(
        String::from("user2@example.com"),
        String::from("user2"),
    );
    println!("User 2 active status: {}", user2.active);

    // Creating instances from other instances with struct update syntax
    let user3 = User {
        email: String::from("user3@example.com"),
        username: String::from("user3"),
        ..user1 // The `..` syntax specifies that the remaining fields not explicitly set
                // should have the same value as the fields in the given instance.
    };
    println!("User 3 sign in count: {}", user3.sign_in_count);


    // Using Tuple Structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("The color is black: R={}, G={}, B={}", black.0, black.1, black.2);
    println!("The origin point is: x={}, y={}, z={}", origin.0, origin.1, origin.2);
}

// A function that returns a User instance
fn build_user(email: String, username: String) -> User {
    User {
        email, // Using the field init shorthand syntax
        username, // because the parameter names are the same as the struct field names
        active: true,
        sign_in_count: 1,
    }
}
