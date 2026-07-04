/*Smart Pointers in Rust

Smart pointers are data structures that act very much like a pointer,
but also have additional metadata and capabilities.

The Box<T> type allows you to store data on the heap rather than the stack.

Boxes don't have performance overhead, other than storing their data on the heap instead of on the stack.

It's useful in 3 situations:
1. When you have a type whose size can't be known at compile time
and you want to use a value of that type in a context that requires an exact size.

Example: Implement the Recursive Type with Boxes (1, (2, (3, Nil)))

2. When you have a large amount of data and you want to transfer ownership
but ensure the data won't be copied when you do so.

3. When you want to own a value and you care only that it's a type that implements a particular trait
rather than being of a specific type.

*/

// Recursive Type with Boxes
// (1, (2, (3, Nil)))

use crate::List::{Cons, Nil};

enum List{
    Cons(i32, Box<List>),
    Nil
}

fn main() {
    let list: List = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
