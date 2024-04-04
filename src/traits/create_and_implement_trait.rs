pub trait Summary {
    fn summarize(&self) -> String;

    fn summarize_author(&self) -> String;

    // we can use functions which has to be implemented in functions with default realization
    fn summarize_default(&self) -> String {
        format!("Read more from {}", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn summarize_author(&self) -> String {
        String::from("Article author")
    }
    // summarize_default wasn't implemented because it has default value
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }

    fn summarize_author(&self) -> String {
        String::from("Tweet author")
    }
}
