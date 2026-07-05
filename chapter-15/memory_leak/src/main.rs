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


// To understand Rc and RefCell, let’s stick with our playground analogy.

// In Rust, the computer is a very strict parent. Usually, the rule is: "Only one person can own a toy at a time." If you want to give the toy to someone else, you have to hand it over completely.

// Rc and RefCell are special tools that let us bend those strict rules safely.

// 👥 1. Rc (The Sharing Club)
// Rc stands for "Reference Counting."

// Imagine you and your friends find a really cool toy, but Rust's strict rules say only one person can own it. You use an Rc box to solve this.

// How it works: Rc acts like a club membership. When you put a toy inside an Rc box, multiple kids can hold a special "clone" remote control to it.

// The Counter: Every time a new kid gets a remote, the box counts: 1... 2... 3... (strong_count).

// Going Home: When a kid leaves the playground, they throw away their remote, and the count goes down. When the count hits 0 (nobody is playing with it anymore), the toy is finally cleaned up and put away.

// ⚠️ The Catch: Rc lets everyone look at the toy, but it has a golden rule: Look, but don't touch (modify)! It makes the toy completely frozen/read-only.

// 🔒 2. RefCell (The Playground Guard)
// What if you need to change the toy? (For example, in your code, changing who Andy's next friend is). Since Rc froze the toy, you need RefCell.

// RefCell acts like a strict playground guard.

// Interior Mutability: It allows you to change things inside a frozen box.

// The Guard's Rule: The guard says, "Fine, you can change the toy, but only one person can write on it or change it at any given moment."

// borrow() and borrow_mut():

// If you want to just look at it, you ask the guard to borrow() it. Multiple kids can look at once.

// If you want to change it, you ask to borrow_mut() (borrow with permission to mutate/change). The guard makes sure absolutely nobody else is looking or touching it while you do this.

// 🤝 Putting Them Together: Rc<RefCell<T>>
// In your code, you see them combined like this: Rc::new(Node { next: RefCell::new(...) }).

// When you combine them, you get the ultimate superpower: Multiple people can own the object (Rc), AND those people are allowed to change it (RefCell).

// 💥 Why did the code crash earlier?
// Remember when the guard (RefCell) said only one person can change it at a time?

// In this line of code:

// Rust
// println!("a next = {:?}", a.next.borrow());
// The computer asks to borrow() Andy's notes. While holding that borrow open, it tries to read Ben, who tries to read Andy, creating that infinite loop loop we talked about. Because the first borrow was never closed, the guard gets confused, and the program breaks!

// By adding the fix:

// Rust
// let weak_opt = a.next.borrow().clone(); // Look quickly and copy it!
// You quickly look at the notes, copy the info down, and immediately hand the notes back to the guard. Now the guard is happy, and you can safely read your copy without getting stuck!


// 🔍 Line-by-Line Breakdown
// Rust
// if let Some(weak: Weak<Node>) = weak_opt {
// What it means: "Hey, let's open up that note we copied down. Is there actually a pinky promise (Weak) written inside it?"

// If the note is empty (None), the computer skips everything. If there is a pointer there (Some), it names it weak and moves to the next step.

// Rust
// if let Some(strong: Rc<Node>) = weak.upgrade() {
// What it means: This is the magic step! A pinky promise (Weak) isn't strong enough to let you actually talk to someone—they might have already gone home. So, you try to upgrade() it.

// You are asking the playground: "Hey, is the friend this pinky points to still on the playground? If they are, let me temporarily turn this into a real, solid high-five (Rc)."

// If the friend is still there, it successfully creates strong (a real Rc pointer).

// Rust
// println!("b next = {:?}", strong.next.borrow());
// What it means: Because you successfully upgraded to a real high-five (strong), you can now safely look at what Node B (strong) is holding! You ask the playground guard (borrow()) to let you read Node B's notes.

// Rust
// } else {
//     println!("b next is None");
// }
// What it means: This is your safety net. If weak.upgrade() failed, it means the friend packed up their bags and went home already. Instead of crashing, the computer calmly says, "Oh, looks like they are gone!" and prints out b next is None.

// 🏆 Why this is so smart
// By using weak.upgrade(), you avoid the infinite loop entirely. You only get a real Rc pointer temporarily to look at Node B, and as soon as this if let block finishes, that temporary high-five is thrown away. No forever-loops, no angry playground guards, and no computer crashes!