// The same memory rules apply to vector
// If the vector stop exist it's going to be destroyed along with all objects inside

fn create_vector() {
    // tell compiler what values we're going to store
    let v: Vec<i32> = Vec::new();

    // automatically identify values using vec! macro
    let macro_v = vec![1, 2, 3];
}

fn change_vector() {
    // automatically identify vector type based on the value in push operation
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
}

fn get_values_from_vector() {
    let v = vec![1, 2, 3, 4, 5];

    // will panic if the index out of bound
    let third: &i32 = &v[2];
    println!("Third element is {}", third);

    // return Option None if the index out of bound
    let third: Option<&i32> = v.get(2);
    match third {
        None => println!("There is no third element"),
        Some(third) => println!("Third element is {}", third),
    }
}

fn iterate_over_vector() {
    let v = vec![100, 32, 57];

    // non-mutable links
    for i in &v {
        println!("{i}");
    }

    let mut v = vec![100, 32, 57];

    // mutable links
    for i in &mut v {
        // add 50 to each element in vector
        *i += 50;
    }
    println!("Changed vector {:?}", v);
}

fn enums_to_store_different_types() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String)
    }
    let v = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(15.4),
        SpreadsheetCell::Text(String::from("example"))
    ];
    for i in &v {
        match i {
            SpreadsheetCell::Int(num) => println!("This is a number {num}"),
            SpreadsheetCell::Float(fl) => println!("This is a flow number {fl}"),
            SpreadsheetCell::Text(tx) => println!("This is a text {tx}"),
        }
    }
}
