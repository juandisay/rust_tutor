// Enums, or enumerations, allow you to define a type by enumerating its possible variants.

// Defining an enum
// This enum has four variants with different types and amounts of associated data.
#[derive(Debug)] // This attribute lets us print the enum for debugging
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// We can also define methods on enums
impl Message {
    fn call(&self) {
        // method body would be defined here
        println!("Called method on: {:?}", self);
    }
}

// The `Option` enum
// The standard library defines `Option<T>`, which is used when a value could be something or it could be nothing.
// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    let m1 = Message::Write(String::from("hello"));
    m1.call();

    let m2 = Message::Move{ x: 10, y: 20 };
    m2.call();

    // The `match` control flow operator
    // `match` allows you to compare a value against a series of patterns and then execute code
    // based on which pattern matches.
    let coin = Coin::Quarter(UsState::Alaska);
    println!("Value of coin: {} cents", value_in_cents(coin));

    // `match` with `Option<T>`
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("Five plus one is {:?}", six);
    println!("None plus one is {:?}", none);

    // The `if let` syntax is a concise way to handle a single `match` pattern.
    let some_u8_value = Some(3u8);
    if let Some(3) = some_u8_value {
        println!("three!");
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... etc
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
