mod aggregator;

use aggregator::{SocialPost, NewsArticle, Summary};

fn main() {
    let post = returns_summarizable();
    println!("1 new post: {}", post.summarize());

    let article = NewsArticle {
        headline: String::from("penguins win the Stanley Cup Championshp!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The pittsburgh Penguins once again are the best \
            hockey team in the NHL.",
        ),
    };

    println!("News article available! {}", article.summarize());
}

fn returns_summarizable() -> impl Summary {
    SocialPost {
            username: String::from("horse_ebooks"),
            content: String::from (
                "of course, as you probably already know, people",
            ),
            reply: false,
            repost: false,
        }
}