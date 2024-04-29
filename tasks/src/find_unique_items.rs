use std::collections::HashSet;
use std::hash::Hash;

fn unique<'a, T>(arr: &'a Vec<T>) -> Vec<T>
    where T: Eq + Hash + Clone + 'a {
    let mut unique_elements: HashSet<&'a T> = HashSet::new();
    unique_elements.extend(arr.iter());
    unique_elements.into_iter().cloned().collect()
}

#[test]
fn empty_list() {
    let input: Vec<i32> = vec![];
    let expected_output: Vec<i32> = vec![];
    let actual_output = unique(&input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1, 4, 5];
    let mut expected_output = vec![1, 4, 5];
    let mut actual_output = unique(&input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1, 5, 2];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(&input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list_with_duplicates() {
    let input = vec![1, 5, 2, 2, 1];
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(&input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list_with_duplicates() {
    let mut input = vec![1, 5, 2, 2, 1];
    input.sort_by(|x, y| x.partial_cmp(y).unwrap());
    let mut expected_output = vec![1, 2, 5];
    let mut actual_output = unique(&input);
    expected_output.sort_unstable();
    actual_output.sort_unstable();
    assert_eq!(actual_output, expected_output);
}
