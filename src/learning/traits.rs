pub mod create_and_implement_trait;
pub mod traits_as_parameters;
pub mod traits_with_generics;
pub mod return_traits;

use create_and_implement_trait::{Tweet, Summary, NewsArticle};

pub fn test_traits() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    println!("The default message: {}", article.summarize_default());
}
