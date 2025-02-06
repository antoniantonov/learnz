use std::{collections::HashMap, default};

fn main() {
    println!("Hello, world!");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // Accessing the value
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Score for team {}: {}", team_name, score);

    // Iterate over all the elements
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    // Ownership - key is moved into the insert method and value as well.
    let key = String::from("ThisIsKey");
    let value = String::from("ThisIsValue");
    let mut scores1 = HashMap::new();
    scores1.insert(key, value);

    // This won't compile
    // println!("Key: {}, Value: {}", key, value);

    // Inserting references - the lifetime of the reference should be greater than the lifetime of the HashMap
    let value = String::from("ThisIsValue");
    let mut scores2: HashMap<&str, &String> = HashMap::new();
    scores2.insert("Key1", &value);

    // Updating the values
    let default_value_key1 = String::from("Default_Key1");
    let default_value_key2 = String::from("Default_Key2");
    // Here we need to define the values because they will be borrowed by the HashMap
    // If we defined it as `.or_insert(&String::from("Default"));` - it does not compile, because the temp value will be dropped.
    // And we need it's lifetime for the lifetime of the hashmap.
    scores2.entry("Key1").or_insert(&default_value_key1);
    scores2.entry("Key2").or_insert(&default_value_key2);
    println!("{scores2:?}");

}
