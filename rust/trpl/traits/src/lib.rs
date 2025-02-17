
use std::fmt::Display; 

pub trait Summary {
    fn summarize_author(&self) -> String;

    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
// Use the default implementation of the Summary trait.
impl Summary for NewsArticle { 
    fn summarize_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize_author(&self) -> String {
        format!("@{}", self.username)
    }
}

// We can have 2 separate types that implement the same trait.
pub fn notify(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
}

// If we want the same type for both parameters, we can use a trait bound.
pub fn notify2<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} and {}", item1.summarize(), item2.summarize());
}

pub struct Pair<T> {
    pub x: T,
    pub y: T,
}

// Implementing the function that returns new instance of Pair<T>.
impl<T> Pair<T>{ 
    pub fn new (x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

impl<T: Summary> Pair<T> {
    pub fn cmp_display_summary(&self) {
        if self.x.summarize() == self.y.summarize() {
            println!("The two are the same.");
        } else {
            println!("The two are different.");
        }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    pub fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}