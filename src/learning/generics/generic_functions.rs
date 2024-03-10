// to compare we need to use arguments with type std::cmp::PartialOrd (Comparable)
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

pub fn find_largest_number() -> i32 {
    let nums = vec![3, 5, 1, 123, 42, 2];
    *largest(&nums)
}

pub fn find_largest_char() -> char {
    let chars = vec!['a', 'z', 'y', 'b', 'm', 't'];
    *largest(&chars)
}
