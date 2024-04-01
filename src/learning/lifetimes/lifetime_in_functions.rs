/*
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
*/
pub fn example() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}

pub fn example_with_different_lifetimes_compiled() {
    let s1 = String::from("abcd");
    {
        let s2 = String::from("abc");
        let result = longest(&s1, &s2);
        println!("The longest string is {result}");
    }
}

pub fn example_with_different_lifetimes_non_compiled() {
    let s1 = String::from("Abcd");
    let result;
    {
        // it's not able to compile because the minimum lifetime is here
        let s2 = String::from("abc");
        result = longest(&s1, &s2);
    }
    // here s2 is not valid meaning min all variables will be dropped by a compiler
    // therefore result is not valid here because it has time-to-live equal to s2
    println!("The longest string is {result}");
}

// the min lifetime of x and y will be picked up by a compiler
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn non_compiled_example() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    // Non-compiled
    // println!(r);
}
