/*
Interior Mutability Pattern

Interior mutability is a design pattern in Rust that allows you to 
mutate data even when there are immutable references to that data.

Borrowing rules recap:
1. At any given time, you can have either ONE mutable reference OR any number of immutable references.
2. References must always be valid.

With standard references and Box<T>, these rules are enforced at COMPILE TIME.
With RefCell<T>, these rules are enforced at RUNTIME. If you break the rules, your program will panic!
Note: RefCell<T> is strictly for single-threaded contexts.
*/
// first example: basic Refcell
// second example: RefCell for an API Tracker Limiter
// third example: RefCell and Rc
use std::cell::RefCell;


fn main() {
    println!("--- Example 1: Basic RefCell ---");
    first_eg();

    println!("\n--- Example 3: RefCell combined with Rc ---");
    third_eg();

    println!("\n--- Similar to example 3 ---");
    similar_third_eg();

}

// =========================================================================
// FIRST EXAMPLE: Basic RefCell Mechanics
// =========================================================================
fn first_eg() {
    // Wrap an immutable binding around a RefCell holding the value 14
    let number = RefCell::new(14); 

    {
        // Mutably borrow the inner data at runtime. 
        // This returns a RefMut smart pointer, allowing us to mutate it.
        let mut mutable_borrow = number.borrow_mut(); 
        *mutable_borrow += 1; // Increment 14 to 15
    } // 'mutable_borrow' goes out of scope here, releasing the runtime mutable lock!

    // Immutably borrow the inner data to read it safely
    println!("number: {}", number.borrow());
}

// =========================================================================
// SECOND EXAMPLE: RefCell for an API Tracker Limiter
// =========================================================================
pub trait Messenger {
    // Note the immutable reference '&self'. 
    // Implementing structs must find a way to mutate their internal logs using this!
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T, 
    value: usize,     
    max: usize,       
}

impl<'a, T> LimitTracker<'a, T> where T: Messenger {
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max: f64 = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger.send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: You've used up over 75% of your quota!");
        }
    }
}

// =========================================================================
// THIRD EXAMPLE: Multiple Owners of Mutable Data (Rc + RefCell)
// =========================================================================
fn third_eg() {
    use std::rc::Rc;
    // Rc allows multiple owners. RefCell allows mutability. 
    // Combining them gives you multiple owners that can ALL mutate the shared value!
    let value = Rc::new(RefCell::new(5));

    // Clone the pointer for Owner A and Owner B
    let owner_a = Rc::clone(&value);
    let owner_b = Rc::clone(&value);

    // Owner A mutates the value
    *owner_a.borrow_mut() += 10;

    // Owner B can immediately see and mutate the updated value
    *owner_b.borrow_mut() += 20;

    // Print from original pointer
    println!("Final shared value: {}", value.borrow()); // Output: 35
}

// =========================================================================
// Similar to Third example 
// =========================================================================

use crate::List::{Cons, Nil};
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

fn similar_third_eg() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));

    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));


    //print the value
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

    *value.borrow_mut() = 10; // change the value to 10
    // print the value again
    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("c: {:?}", c);

}

// =========================================================================
// TESTS FOR THE API LIMIT TRACKER
// =========================================================================
#[cfg(test)]
mod test {
    use super::*;

    struct MockMessenger {
        // We use RefCell here so we can mutate this vector inside an immutable `send` function!
        send_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                send_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Interior Mutability: .borrow_mut() allows us to push data 
            // even though '&self' is completely immutable!
            self.send_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        // 80 out of 100 is 80%, triggering the >= 0.75 condition
        limit_tracker.set_value(80);

        // Use .borrow() immutably to check if a message was safely appended
        assert_eq!(mock_messenger.send_messages.borrow().len(), 1);
        assert_eq!(
            mock_messenger.send_messages.borrow()[0],
            "Warning: You've used up over 75% of your quota!"
        );
    }
}

/*
The real magic of RefCell shines in the MockMessenger test case. The Messenger
trait dictates that fn send(&self, msg: &str) must take an immutable &self.
Normally, you could never keep a historical list of sent messages inside a mock
struct because you can't .push() to a vector without &mut self.

By wrapping your vector in a RefCell<Vec<String>>, you bypass the compiler check,
safely borrow the vector mutably at runtime via .borrow_mut(), and record your data
perfectly.
*/