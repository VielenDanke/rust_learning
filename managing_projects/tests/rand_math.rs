mod test_utils;

use managing_projects;
use managing_projects::is_even;

#[test]
fn is_even_integration() {
    for i in 0..1000 {
        assert_eq!(is_even(i), i % 2 == 0, "The number {} is noto even", i);
    }
}
