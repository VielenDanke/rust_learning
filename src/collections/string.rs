fn create_string() {
    let mut s = String::new();

    let data = "initial content";
    let s = data.to_string();
    let s = "initial content".to_string();
    let s = String::from("initial content");
}

fn push_to_string() {
    let mut s = String::from("foo");
    // push another string
    s.push_str("bar");
    // push character
    s.push('t');
}

pub fn union_strings() {
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    // the reason of &s2 is under the hood + str is using fn add(self, s: &str) -> String function
    // which requires link to variable (avoiding borrowing)
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    let result = format!("{s3}-{s2}");
    println!("{result}");
}

pub fn indexing_string(idx: usize) -> i32 {
    // rust is not able to index strings because how the language is storing it
    let data = String::from("data is data");
    let data_bytes = data.as_bytes();
    println!("{}", data_bytes[0]);
    match data_bytes.get(idx) {
        None => -1,
        Some(ch) => *ch as i32
    }
}

pub fn string_slices() {
    let hello = "Здравствуйте";
    // if we use &hello[0..1] rust will panic because it's impossible to fit the whole first letter into 1 byte
    // be careful when using slices and strings in Rust
    let s = &hello[0..4];
}

pub fn iteration_over_strings() {
    let data = "Здравствуйте";

    for ch in data.chars() {
        println!("Letter {ch}");
    }
    for b in data.bytes() {
        println!("Byte {b}");
    }
}
