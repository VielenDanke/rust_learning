
// sum takes ownership on v1_iter after a call to .sum()
// like terminal methods
pub fn consume_iterators() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    println!("Sum of elements {:?} is equal to {}", v1, v1_iter.sum());
}

// non-terminal methods, the terminal methods has to be called in order to trigger chain of lazy methods
pub fn do_not_consume_iterators() {
    let v1 = vec![1, 2, 3];

    let new_list_v1 = v1.iter().map(|x| x + 1).collect();

    println!("List {:?} contains all values {:?} incremented by 1", new_list_v1, v1);
}

pub fn iterator_catch_variables() {
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let expected_size = 10;

    let shoes = vec![
        Shoe {
            size: expected_size,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: expected_size,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, expected_size);

    println!("Shoes of my size {} are {:?}", expected_size, in_my_size);
}
