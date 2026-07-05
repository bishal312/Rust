/*
+---------------+      +---------------+
|               |  Rc  |               |
|    Node A     +------>    Node B     |
|               |      |               |
+-------+-------+      +-------+-------+
        ^                      |
        |                      |
        |          Rc          |
        +----------------------+
*/

// A Memory leak is created here because of the cyclic reference between a and b.
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    name: String,
    next: RefCell<Option<Weak<Node>>>,
}

fn main() {
    let a: Rc<Node> = Rc::new(Node {
        name: "A".to_string(),
        next: RefCell::new(None),
    });

    let b: Rc<Node> = Rc::new(Node {
        name: "B".to_string(),
        next: RefCell::new(Some(Rc::downgrade(&a))),
    });

    *a.next.borrow_mut() = Some(Rc::downgrade(&b)); // creates a cycle

    println!("a strong= {}", Rc::strong_count(&a));
    println!("a strong = {}", Rc::strong_count(&b));

    // At this point a and b have cyclic references and won't be dropped.

    //Stack overflow
    println!("a next = {:?}", a.next.borrow());

    //fix: end borrow early
    let weak_opt: Option<Weak<Node>> = a.next.borrow().clone();

    if let Some(weak: Weak<Node>) = weak_opt {
        if let Some(strong: Rc<Node>) = weak.upgrade() {
            println!("b next = {:?}", strong.next.borrow());
        } else {
            println!("b next is None");
        }
    }
}

// Imagine you have two friends, Andy (Node A) and Ben (Node B). They are playing a game of tag, but with a funny rule: they have to hold onto each other's shirts.

// 🤝 The Endless Loop (The Problem)
// Andy reaches out his hand and grabs Ben's shirt.

// Ben reaches out his hand and grabs Andy's shirt.

// Now, they are standing in a perfect circle, holding onto each other!

// Normally, when playtime is over, the computer expects everyone to let go so they can go home (this is called cleaning up memory). But there is a strict rule in this playground: You can only let go of your friend's shirt AFTER they let go of yours first.

// Because Andy is waiting for Ben to let go, and Ben is waiting for Andy to let go, neither of them can ever leave! They are stuck out on the playground forever. In computer talk, this is called a Memory Leak (or a cyclic reference).

// 😵 The "Infinite Question" (The Stack Overflow)
// The code tries to print out what Andy is holding.

// The computer asks: "Andy, who are you holding?"

// Andy says: "I'm holding Ben! Hey Ben, who are you holding?"

// Ben says: "I'm holding Andy! Hey Andy, who are you holding?"

// Andy says: "I'm holding Ben! Hey Ben..."

// They keep whispering back and forth forever and ever until the computer gets dizzy, runs out of breath, and crashes! This is what the code calls a Stack Overflow.

// 🛠️ The Clever Fix
// To fix this, the code uses something called a Weak pointer (think of it like a "pinky promise" instead of a tight grip).

// Instead of grabbing shirts tightly, they just point a pinky finger at each other. Because it's just a weak pinky promise, the computer is allowed to say, "Okay kids, playtime is over, time to go home!" even if they are still pointing at each other.

// At the very end, the code carefully checks: "Hey, is that pinky finger still pointing at a real friend, or did they already go home?" If the friend is still there, it temporarily upgrades to a real high-five (weak.upgrade()) to talk to them safely without getting stuck in an infinite loop!