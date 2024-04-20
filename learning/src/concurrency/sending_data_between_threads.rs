use std::sync::{Arc, mpsc};
use std::thread;
use std::time::Duration;

// 1 receiver N senders - the channels in Rust

pub fn channel_example() {
    // tx - Sender, rx - Receiver
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = Arc::new(String::from("Hi"));
        tx.send(Arc::clone(&val)).unwrap(); // Return Result<T, E> in case of the receiver is dead and we're not able to send
        println!("{val} is sent");
    });
    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

pub fn multiple_messages_channel_example() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}

pub fn multiple_senders_channel_example() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
}
