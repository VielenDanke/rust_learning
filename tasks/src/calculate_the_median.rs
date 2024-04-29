pub fn median(mut arr: Vec<f32>) -> Option<f32> {
    if arr.is_empty() {
        None
    } else {
        arr.sort_by(|x, y| x.partial_cmp(y).unwrap());
        let n = arr.len();
        let middle = n / 2;
        if n % 2 == 0 {
            Some((arr[middle] + arr[middle - 1]) / 2.0)
        } else {
            Some(arr[middle])
        }
    }
}

#[test]
fn empty_list() {
    let input = vec![];
    let expected_output = None;
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn sorted_list() {
    let input = vec![1.0, 4.0, 5.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn even_length() {
    let input = vec![1.0, 3.0, 5.0, 6.0];
    let expected_output = Some(4.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}

#[test]
fn unsorted_list() {
    let input = vec![1.0, 5.0, 2.0];
    let expected_output = Some(2.0);
    let actual_output = median(input);
    assert_eq!(actual_output, expected_output);
}
