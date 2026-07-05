/*
Concurrency and Parallelism

Concurrency: different parts of a program execute independantly
Parallenlism: different parts of a program execute simultaneously

For simplicity, I will use the term concurrency to refer to both concepts.
we will se some examples of threads too.
*/

use std::thread;
use std::time::Duration;
use std::thread::JoinHandle;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread 1: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    
    // handle.join().unwrap();


    for i in 1..5 {
        println!("Main Thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();


    let v: Vec<i32> = vec![1,2,3,4,5];
    let handle2: JoinHandle<()> = thread::spawn(move || {
        println!("Here is a vector: {v:?}");
    });
    handle2.join().unwrap();
}
