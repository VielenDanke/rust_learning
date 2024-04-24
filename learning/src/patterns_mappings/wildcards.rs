struct Point {
    x: i32,
    y: i32,
    z: i32,
}

pub fn example_wildcard_arguments(_: i32, y: i32) {
    println!("This code only uses the y parameter: {y}");
}

pub fn example_wildcard_match() {
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {setting_value:?}");
}

pub fn example_wildcard_partial() {
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

pub fn example_underscore_variable() {
    let _x = 5; // this variable is ignored, but the value 5 is assigned to _x
    let y = 10;
}

pub fn example_variable_not_moved_with_wildcard() {
    let s = Some(String::from("Hello!"));

    if let Some(_) = s {
        println!("found a string");
    }

    println!("{s:?}");
}

pub fn ignoring_with_double_dot() {
    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {x}"),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
