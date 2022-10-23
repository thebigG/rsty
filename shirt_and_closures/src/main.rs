use shirts::{Inventory, ShirtColor};
use std::thread;
use std::time::Duration;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red],
    };

    let user_pref1 = Some(ShirtColor::Red);
    let giveaway1 = store.giveaway(user_pref1);

    println!(
        "The user with preference {:?} gets {:?}",
        user_pref1, giveaway1
    );

    let user_pref2 = None;
    let giveaway2 = store.giveaway(user_pref2);
    println!(
        "The user with preference {:?} gets {:?}",
        user_pref2, giveaway2
    );

    //Some examples of closures

    let expensive_closure = |num: i32| -> i32 {
        println!("Calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num * 100
    };

    let two_hundred = expensive_closure(2);
    let three_hundred = expensive_closure(3);

    closures_and_move();
}

fn closures_and_move() {
    let list = vec![1, 2, 3];
    println!("Before defining closure");

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();

    // println!("{:?}",list); Does not compile since ownership of list has been moved to another thread
}

// call once closure. Notice the FnOnce(move out of its body and caLL once) trait here.
// There is also FnMut(call more than once, do not move out of its body and may mutate its captures)
//Fn -- More than once, do not move out of body and cannot mutate values
// impl<T> MyOption<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T
//     where
//         F: FnOnce() -> T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }
