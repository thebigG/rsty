use std::collections::HashMap;

fn main() {
    let mut scores: HashMap<String, i32> = HashMap::new();

    let blue_team = String::from("Blue");
    scores.insert(blue_team, 10);
    //println!("{blue_team}"); illegal as this string was moved(changed owner) when it was inserted into the hash map
    scores.insert(String::from("Yellow"), 50);

    scores.entry(String::from("Yellow")).or_insert(50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    println!("Hello, world!");

    for (key, value) in &scores {
        println!("{}:{}", key, value);
    }
}
