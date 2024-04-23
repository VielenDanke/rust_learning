use std::collections::VecDeque;

pub fn example_let_some_while() {
    let mut queue = VecDeque::from(vec![1, 2, 3]);

    while let Some(val) = queue.pop_front() {
        println!("The value is {}", val);
    }
}

pub fn patterns_group() {
    let x = 1;

    match x {
        1 | 2 => println!("One or Two"),
        3 => println!("Three"),
        4..=10 => println!("From 4 to 10 included"),
        _ => println!("Anything"),
    }

    let x = 'c';

    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}

