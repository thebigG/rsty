use std::sync::{LockResult, Mutex};
use std::thread;

fn main() {
    let counter = Mutex::new(0);

    let mut handles:Vec<i32> = vec![];

    for _ in 0..10{
        let handle = thread::spawn(move || {
            let mut numResult = counter.lock();
            let counter = match numResult{
                Ok(num) => {num}
                Err(lock) => {panic!("")}
            };
        });
    }
}
