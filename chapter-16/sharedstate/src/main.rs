/*
Shared-Stated Concurrency

...in the previous lesson, we introduced channels and message passing.
// Using channels, we can send messages between threads,
but we need to be careful about shared state.

// In this lesson, we will learn about shared-state concurrency.
// we will use the Mutex type to protect shared data.

*/
use std::sync::{Mutex, MutexGuard, Arc}; // mutual exclusion; / Atomic reference counting;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    // let counter: Mutex<i32> = Mutex::new(0);
    // let mut handles: Vec<JoinHandle<()>> = vec![];

    // for _ i 0..10 {
    //     let handle: JoinHandle<()> = thread::spawn(move || {
    //         let mut num: MutexGuard<'_, i32> = counter.lock().unwrap();

    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }

    // for handle: JoinHandle<()> in handles {
    //     handle.join().unwrap();
    // }

    // println!("Result: {}", *counter.lock().unwrap());

    
    
    // --- STEP 1: INITIALIZING THE DATA ---
    
    // Wrap the integer '0' in a Mutex for safety, then wrap it in an Arc.
    // At this moment, the Atomic Reference Count is exactly 1.
    let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

    // A collection to hold thread JoinHandles so the main thread can await them later.
    let mut handles: Vec<JoinHandle<()>> = vec![];

    // --- STEP 2: THE SPAWNING LOOP ---
    
    // Spin up 10 separate OS background threads to perform concurrent updates.
    for _ in 0..10 {
        
        // Clone the Arc pointer to increment the reference count.
        // This gives each loop iteration unique, shared ownership of the same heap data.
        let counter: Arc<Mutex<i32>> = Arc::clone(&counter);
        
        // 'move' forces the closure to take absolute ownership of our local 'counter' clone.
        let handle: JoinHandle<()> = thread::spawn(move || {
            
            // --- STEP 3: INSIDE THE THREAD (THE MAGIC HAPPENS) ---
            
            // Request the lock. Blocks (sleeps) the thread if the lock is held by another worker.
            // .unwrap() ensures safety by panicking if a previous thread poisoned the Mutex.
            let mut num: MutexGuard<'_, i32> = counter.lock().unwrap();

            // Dereference the MutexGuard to gain mutable access to the underlying integer.
            *num += 1;
            
            // <--- 'num' (MutexGuard) goes out of scope and drops right here!
            // RAII instantly releases the lock key so the next waiting thread can take over.
        }); 
        
        // --- STEP 4: TRACKING THE THREADS ---
        
        // Save the worker thread's handle back into our vector on the main thread.
        handles.push(handle);
    }

    // --- STEP 5: WAITING FOR COMPLETION ---
    
    // Block the main thread until every background worker has safely checked out.
    // This prevents premature program exit before the concurrent math is complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // --- STEP 6: THE GRAND FINALE ---
    
    // All background threads are dead; Arc ref count is back to 1.
    // Lock one final time to cleanly dereference and print the verified result: 10.
    println!("Result: {}", *counter.lock().unwrap());
}

// To understand how this code works, we need to understand the two wrapper types protecting our integer: Mutex and Arc.

// 1. Mutex<T> (Mutual Exclusion)
// A Mutex is like a locked room with only one key.

// Inside the room is your data (in this case, the integer 0).

// If a thread wants to read or update the data, it must first knock on the door and request the key using .lock().

// Once a thread gets the key, it steps inside and locks the door behind it. Any other thread that tries to access the data will be forced to wait patiently in line until the first thread leaves and returns the key.

// The Catch: A standard Mutex only understands a single owner. If you try to pass a plain Mutex into 10 different threads, Rust’s compiler will stop you because a single value cannot be owned by multiple threads simultaneously.

// 2. Arc<T> (Atomic Reference Counting)
// An Arc is a smart pointer that allows safe, shared ownership across multiple threads.

// It wraps around your data (or your Mutex) and keeps a strict headcount of how many active clones point to that data.

// Every time you call Arc::clone(&counter), it doesn't actually copy the data inside; it just increments the counter by 1. When a thread finishes running, its clone goes out of scope, and the Arc decrements the counter by 1.

// The "A" stands for Atomic. This means the counting mechanism itself is fully thread-safe and can be updated by multiple CPU cores at the exact same millisecond without breaking.


/*
[Main Thread] Creates Arc<Mutex(0)> 
      │
      ├── (Loop 10x) ──► Clones Arc Pointer (Ref count goes up)
      │                     │
      │                     └──► [Spawn Thread] ──► .lock() (Waits in line)
      │                                                │
      │                                                ├──► Acquires MutexGuard
      │                                                ├──► Increments *num += 1
      │                                                └──► Out of scope -> Drops lock
      │
      ├──► Loops through handles.join() (Main thread waits for everyone to finish)
      │
[Main Thread] locks one last time ──► Prints "Result: 10"
*/