use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

fn main() {
    spawn_thread_and_move();
}

fn two_threads_sleep() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }
}
fn two_threads_sleep_and_join() {
    let handle: JoinHandle<_> = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread.", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from main thread.", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();
}

fn spawn_thread_and_move() {
    let v = vec![1, 2, 3];
    //Not using move wil cause a compiler error
    //since v is not guaranteed to be valid for the duration of the spawned thread
    let handle = thread::spawn(move || {
        println!("Here's a vector:{:?}", v);
    });

    handle.join().unwrap();
}
