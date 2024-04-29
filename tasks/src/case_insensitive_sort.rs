fn sort_usernames<T: AsRef<str>>(usernames: &mut Vec<T>) {
    usernames.sort_by_cached_key(|v| v.as_ref().to_lowercase())
}

#[test]
fn five_users() {
    let mut users = vec!["Todd", "Amy", "mike99", "Jennifer", "alison"];
    let sorted = vec!["alison", "Amy", "Jennifer", "mike99", "Todd"];
    sort_usernames(&mut users);

    assert_eq!(users, sorted);
}
