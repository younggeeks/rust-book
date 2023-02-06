use std::fmt::{Debug, Display};

fn main() {
    let tweet = Tweet {
        username: "samjunior".to_string(),
        content: String::from("This is probably a lorem text, should've been an email"),
        reply: false,
        retweet: false,
    };

    notify(&tweet);
}

pub trait Summary {
    fn summarize_author(&self) -> String;
    fn summarize(&self) -> String {
        format!("Read more from {}...", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Debug for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tweet")
            .field("username", &self.username)
            .field("content", &self.content)
            .field("reply", &self.reply)
            .field("retweet", &self.retweet)
            .finish()
    }
}
impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{}", self.username, self.content)
    }
}
impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

pub fn notify<T: Summary + Display + Debug>(item: &T) {
    println!("Breaking news! {}", item.summarize())
}
