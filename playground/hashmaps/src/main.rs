use std::collections::HashMap;

fn main() {

    // Initialize with constructor
    let mut map = HashMap::new();
    map.insert("Blue".to_string(), 10);
    map.insert("Yellow".to_string(), 50);

    // Initialize with collect
    let teams = vec!["Blue", "Yellow"];
    let scores = vec![10, 50];

    let mut map2: HashMap<_, _> =
        teams.into_iter().zip(scores.into_iter()).collect();

    // Get item from HashMap
    // .get(&str) returns an Option, so in this case return Some(&10)
    let current_score = map.get("Blue");
    println!("Blue score is: {}", current_score.unwrap());

    // Iterate over HashMap
    for (key, value) in &map {
        println!("Key: {}, Value: {}", key, value);
    }

    // ==============================

    // UPDATING A HASHMAP

    // Overwriting a value
    // is simple: insert the same key again
    map.insert("Red".to_string(), 10);
    map.insert("Red".to_string(), 25);
    println!("{:?}", map);  // =>> the key 'Red' has the value 25

    // Only inserting a value if the key has no value
    map.entry("Red".to_string()).or_insert(19);
    map.entry("Green".to_string()).or_insert(10);
    println!("{:?}", map);  // =>> key 'Red' mantains the value 25 and add key 'Green' with 10

    // Updating a value based on the old value
    let text = "hello world wonderful world";

    let mut map3 = HashMap::new();

    for word in text.split_whitespace() {
        let count = map3.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map3);
}
