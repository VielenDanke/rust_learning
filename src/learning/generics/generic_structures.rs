/*
In enums we can use generics as well:
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}
 */

struct Point<T> {
    x: T,
    y: T,
}

struct PointCombined<T, U> {
    x: T,
    y: U,
}

fn declare_structure() {
    let point_integer = Point {x: 5, y: 10};
    let point_float = Point {x: 5.0, y: 10.0};
    // let point_combined = Point {x: 5, y: 10.0}; won't compile because of different x and y types
    let point_combined = PointCombined{x: 5, y: 10.0};
}
