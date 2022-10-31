use std::rc::Rc;
use std::sync::{Arc, LockResult, Mutex};
use std::thread;

fn main() {
    // Arc is an atomic(thread-safe) reference-counted reference
    let counter = Arc::new(Mutex::new(0));

    let mut handles: Vec<_> = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut numResult = counter.lock();
            let mut counter = match numResult {
                Ok(num) => num,
                Err(lock) => {
                    panic!("")
                }
            };

            *counter += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
