fn main() {
    mix_match_if_let();
    for_loop_pattern();
    swap();
    some_var();
    multi_pattern();
    ranged_pattern();
}

fn mix_match_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favortite color, {color}, as your background.")
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{top}");
    }
}

fn for_loop_pattern() {
    let v = vec!['a', 'b', 'c'];

    //This is also a pattern
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}

fn swap() {
    let mut a: i32 = 5;
    let mut b: i32 = 6;

    println!("a={a}, b={b}");

    (a, b) = (b, a);

    println!("a={a}, b={b}");
}

fn some_var() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}

fn multi_pattern() {
    let x = 1;

    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn ranged_pattern() {
    let x = 5;

    match x {
        1..=5 => println!("one through five"),
        _ => println!(),
    }

    let c = 'c';

    match c {
        'a'..='j' => println!("'a' through 'j'"),
        'k'..='z' => println!("'k' through 'z'"),
        _ => println!("No match"),
    }
}

fn destruct_enums() {
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the direction {} and the in the y direction {}"
                    x, y);
        }
        Message::Write(text) => {
            println!("Text message: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change color to red {}, green {}, and blue {}", r, g, b);
        }
    }
}
