enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

fn main() {
    println!("Hello, world!");

    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    // let coin = Coin::Dime;

    // match coin{
    //     Coin::Quarter(state) => println!("State quarter from {:?}", state),
    //     _ => count += 1
    // }

    //same as
    //else is the same as _ which is a wild card for anything that none of the arms match
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
