pub struct ImportantExcerpt<'a> {
    pub part: &'a str
}

fn call() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split(".").next().expect("Couldn't find a sentence");
    let i = ImportantExcerpt { part: first_sentence };
}
