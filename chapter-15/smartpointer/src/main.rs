/*
Smart Pointers in Rust

A pointer is a general concept for a variable that contains a memory address (eg: references in Rust).
Smart pointer are data structures that act like apointers but have additional metadata and capabilities.
Example of smart pointers we have seen so far:
- String
- Vec<T>

The smart pointer pattern is implemented using structs.
THey implement the Derf and Drop traits:
- Deref trait allows an instance of the smart pointer struct to be treatd like a regular reference.
- Drop trait allows you to customize the code that is run when an instance of the smart pointer goes out of scope.

Main smart pointers in Rust:
- The Box<T> for allocating valuse on the heap is a smart pointer.
- THe Rc<T> type is a reference counting smart pointer and enables multiple ownership.
- The RefCell<T> type is a smart pointer that allows mutable borrows checked at runtime.

We'll also cover
- interior mutability pattern: a design pattern in Rust that allows you to mutate data
even when there are immutable references to that data.
- reference cycles: a scenario in which two references contain each other, preventing either value from being dropped.
*/

fn main() {
    // Box<T> for allocating values on the heap

    // creating a Box<T> to store an i32 value on the heap instead of the stack
    let b: Box<i32> = Box::new(5);

    println!("b = {}", b);
}
