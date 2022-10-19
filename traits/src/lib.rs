use std::fmt::format;

pub trait Summary {
    fn summarize_author(&self) -> String {
        String::from("")
    }
    fn summarize(&self) -> String {
        String::from("(Read more from {}...)", self.summarize_author())
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
}

//We can extend the functionality of structs/objects that already exist...
impl Summary for Vec<i32> {
    fn summarize(&self) -> String {
        format!("This is a vec of {} elements", self.len())
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

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
//Same as
// pub fn notify<T: Summary>(item: &T) {
//     println!("Breaking news! {}", item.summarize());
// }

//This says that item must implement both traits Summary and Display
// pub fn notify(item: &(impl Summary + Display)) {

//fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
// same as
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//     T: Display + Clone,
//     U: Clone + Debug,
// {

fn returns_summarizable<T: Summary>() -> T {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

//  fn returns_summarizable() -> impl Summary{
//         Tweet {
//         username: String::from("horse_ebooks"),
//         content: String::from(
//             "of course, as you probably already know, people",
//         ),
//         reply: false,
//         retweet: false,
//     }
// }
