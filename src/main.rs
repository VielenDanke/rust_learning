pub mod learning;

use learning::generics::generic_methods::*;

fn main() {
    let point_1 = MixedPoint {
        x: 1.0,
        y: 15.4,
    };
    let point_2 = MixedPoint {
        x: "bla",
        y: 'c'
    };
    println!("{:?}", point_1.mixup(point_2))
}
