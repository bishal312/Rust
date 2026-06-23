trait Summary {
    fn summarize(&self) -> String;
    // here we can also give a default option like:
    // format!("Read More")
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}
// here we can see to apply this function we have to create two function but we can summarize them using traits

// fn news_aggregator_tweet(tweet: Tweet) {
//     println!("There is a new news in market");
//     println!(
//         "The news is that {} and it is published by {}",
//         tweet.content, tweet.username
//     );
// }

// fn news_aggregator_news(news: NewsArticle) {
//     println!("There is a new news in market");
//     println!(
//         "The news is that {} and it is published by {}",
//         news.content, news.author
//     );
// }

impl Summary for Tweet {
    fn summarize(&self) -> String {
        let content = format!("Tweet by {}: {}", self.username, self.content);
        content
    }
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        let content = format!("News by {}: {}", self.author, self.content);
        content
    }
}

fn news_aggregator(source: &impl Summary) {
    println!("There is a new news in market");
    println!("{}", source.summarize());
}

fn main() {
    println!("Traits function");

    let tweet: Tweet = Tweet {
        username: String::from("bishalkunwarmagar"),
        content: String::from("That there is an amazing rust series launch"),
        reply: false,
        retweet: false,
    };

    let otrtweet: Tweet = Tweet {
        username: String::from("randomuser"),
        content: String::from("That there is an amazing rust series launch"),
        reply: false,
        retweet: false,
    };

    let news_article: NewsArticle = NewsArticle {
        author: String::from("bishal kunwar magar"),
        content: String::from("A new rust series is launched"),
        headline: String::from("Now every one can bea a rust developer!"),
        location: String::from("Butwal"),
    };

    news_aggregator(&tweet);
    news_aggregator(&news_article);

    mixup_news(&tweet, &otrtweet);
    println!("{}", tweet.summarize());
}

// fn mixup_news<T: Summary>(primary: &T, other: &T) {
//     println!("{} and {}", primary.summarize(), other.summarize());
// }

fn mixup_news(primary: &impl Summary, other: &impl Summary) {
    println!("{} and {}", primary.summarize(), other.summarize());
}
