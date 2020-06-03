use std::fmt::{Display, Debug};

pub trait Summary {
    fn summarize(&self) -> String;
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
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Test News"),
        location: String::from("Porto Alegre - RS"),
        author: String::from("Lucas"),
        content: String::from("[...]")
    };

    println!("New article: {}", article.summarize());
}

// Another examples:

fn notify(item: impl Summary) {
    println!("Breaking news: {}", item.summarize());
}


// Trait bound syntax
pub fn notify2<T: Summary>(item: T) { /* function here */ }

// Multiple trait bounds with '+'
fn function1<T: Summary + Display>(item: T) { /* function here */ }

// Multiple trait bounds with 'where'
fn function2<T, U>(x: T, y: U)
    where T: Display + Clone,
          U: Clone + Debug { /* function here */ }

// Returning types that implement traits
fn ret() -> impl Summary {
    Tweet {
        username: "sdf".to_string(),
        content: "sdf".to_string(),
        reply: false,
        retweet: true
    }
}
