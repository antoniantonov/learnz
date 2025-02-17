use traits::{
    Summary,
    Tweet,
    NewsArticle,
};

fn main() {
    println!("Hello, world!");

    let tweet = Tweet {
        username: String::from("from_horse_ebooks"),
        content: String::from("Of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let news_article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best \
            hockey team in the NHL."),
    };

    println!("New article available! {}", news_article.summarize());
}


