use std::process::exit;
use std::sync::mpsc;
use std::sync::mpsc::{RecvError, SendError};
use std::thread;
use std::time::Duration;

fn main() {
    one_sender_multiple_receivers_multiple_values();
}

fn one_sender_receiver() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        match tx.send(val) {
            Ok(_) => {}
            Err(_) => {
                println!("Error on transmit");
            }
        };
    });

    let received = rx.recv();

    let r = match received {
        Ok(r) => r,
        Err(_) => exit(0),
    };

    println!("{:?}", r);
}
fn one_sender_receiver_multiple_values() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let fruits = vec![
            String::from("Apple"),
            String::from("Orange"),
            String::from("Berry"),
        ];

        for fruit in fruits {
            match tx.send(fruit) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error on transmit");
                }
            };
        }
    });

    for received in rx {
        //     let r: String = match received {
        //     Ok(r) => { r }
        //     Err(_) => { exit(0) }
        // };
        println!("{:?}", received);
    }
}
fn one_sender_multiple_receivers_multiple_values() {
    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();
    thread::spawn(move || {
        let fruits = vec![
            String::from("Apple"),
            String::from("Orange"),
            String::from("Berry"),
        ];

        for fruit in fruits {
            match tx.send(fruit) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error on transmit");
                }
            };
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let veggies = vec![
            String::from("Carrot"),
            String::from("pepper"),
            String::from("onions"),
        ];

        for veggie in veggies {
            match tx2.send(veggie) {
                Ok(_) => {}
                Err(_) => {
                    println!("Error on transmit");
                }
            };
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        //     let r: String = match received {
        //     Ok(r) => { r }
        //     Err(_) => { exit(0) }
        // };
        println!("{:?}", received);
    }
}
