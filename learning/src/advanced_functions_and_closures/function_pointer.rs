/// function pointers implemented Fn, FnMut and FnOnce and could be used where the closures are passed as arguments
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

pub fn example() {
    let answer = do_twice(add_one, 1);
    println!("The answer is: {answer}");
}

pub fn example_fn_as_closure() {
    let vec_example = vec![1, 2, 3];
    vec_example.iter().map(|i| i.to_string()).collect::<String>();
    vec_example.iter().map(ToString::to_string).collect::<String>();
}

pub fn example_enum_init_as_fn() {
    enum Status {
        Value(u32),
        Stop,
    }
    let _: Vec<_> = (0u32..20).map(Status::Value).collect();
}
