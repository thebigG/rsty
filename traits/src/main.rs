use traits::{Summary, Tweet};

fn main() {
    let t = Tweet {
        username: "horse_books".to_string(),
        content: "of course, as you already know, people".to_string(),
        reply: false,
        retweet: false,
    };

    let list = vec![1, 2, 3];

    println!("1 new tweet: {}", t.summarize());

    println!("summary of vec: {}", list.summarize());
}
