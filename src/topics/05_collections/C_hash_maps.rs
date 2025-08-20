// The `HashMap<K, V>` type stores a mapping of keys of type `K` to values of type `V`.
// It uses a hashing function to determine how to place these keys and values into memory.

use std::collections::HashMap;

fn main() {
    // Creating a new hash map
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("Scores: {:?}", scores);

    // Accessing values in a hash map
    let team_name = String::from("Blue");
    let score = scores.get(&team_name); // get returns an Option<&V>
    println!("Score for Blue team: {:?}", score);

    // Iterating over a hash map
    println!("Iterating over scores:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // Overwriting a value
    scores.insert(String::from("Blue"), 25);
    println!("Updated scores: {:?}", scores);

    // Only inserting a value if the key has no value
    scores.entry(String::from("Yellow")).or_insert(60);
    scores.entry(String::from("Red")).or_insert(100);
    println!("Scores after using entry: {:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("Word count: {:?}", map);
}
