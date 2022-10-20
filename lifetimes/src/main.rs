use std::fmt::Display;

fn main() {
    let result;
    let string1 = String::from("long string is long");

    //Making the lifetime of result smaller is illegal as we told the compiler that it ought to be
    //the same as string1 and string2
    // {
    let string2 = String::from("xyz");
    result = longest(string1.as_str(), string2.as_str());
    //}
    println!("The longest string is {}", result);

    //static means the lifetime could be the entire duration of this program
    let s: &'static str = "I have a static lifetime.";
}

//This will not compile--we need generic lifetimes for this fn as its shown on the next fn
// fn longest(x: &str, y: &str) -> &str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: &'a T) -> &'a str
where
    T: Display,
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
