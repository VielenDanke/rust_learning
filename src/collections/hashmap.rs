use std::collections::HashMap;

pub fn creation() -> HashMap<String, i32> {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores
}

pub fn access(team_name: &String) {
    let map = creation();

    let score = map.get(team_name).copied().unwrap_or(0);

    println!("{score}");
}

pub fn iteration() {
    let scores = creation();

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}

pub fn ownership() {
    // if variable is able to copy automatically like i32, nothing to worry about
    // if variable is String and is not copied automatically - the HashMap becomes owner of the variable
    let mut scores = creation();

    let team_name = String::from("Red");
    let score = 15;
    scores.insert(team_name, score);
    // we aren't able to use team_name here, ownership went to HashMap
}

pub fn update() {

}

fn update_rewrite_old_values() {
    let mut scores = creation();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);

    println!("{:?}", scores);
}

fn insert_if_no_key_in_map() {
    let mut scores = creation();

    scores.entry(String::from("Blue")).or_insert(50);
    // insert Red if it's not in hash map
    scores.entry(String::from("Red")).or_insert(50);

    println!("{:?}", scores);
}

fn new_value_based_on_old() {
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
