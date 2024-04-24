pub fn example_extra_condition_in_match() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 && x > 2 => {}
        Some(x) => {}
        None => {}
    }

    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(n) if n == y => println!("Matched, n = {n}"),
        _ => println!("Default case, x = {x:?}"),
    }

    println!("at the end: x = {x:?}, y = {y}");
}

pub fn example_multiple_pattern_condition() {
    let x = 4;
    let y = false;

    match x {
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
