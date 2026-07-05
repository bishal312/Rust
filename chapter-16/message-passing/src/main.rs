/*
Using Message Passing to transfer data between threads

Rust's Standard library provides a way to transfer data between threads using channels
A channel is a way to send messages between threads.
It consists of a sender and a receiver.
The sender is used to send messages, and the receiver is usid to recieve messages.
The sender and receiver are created using the `channel` function.
The `channel` function returns a tuple containing the sender and receiver.
*/

//Example 1
use std::sync::mpsc; // multiple producer single consumer;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val: String = String::from("Hello from the thread!");
        tx.send(val).unwrap();
    });

    let received: String = rx.recv().unwrap();
    println!("Received: {}", received);

    // next example

    let (s1, r1) = mpsc::channel();

    thread::spawn(move || {
        let vals: Vec<String> = vec![
            String::from("Hello! "),
            String::from("from "),
            String::from("Bishal "),
            String::from("Magar!"),
        ];

        for val in vals {
            s1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received2 in r1{
        println!("{}", received2);
    }
}
