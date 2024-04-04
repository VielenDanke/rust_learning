mod common;

use rust_learning_lib::structures::*;

#[test]
fn test() {
    common::setup();

    let larger = Rectangle {
        width: 8,
        height: 7,
    };
    let smaller = Rectangle {
        width: 4,
        height: 3,
    };
    assert!(larger.can_hold(&smaller));
}
