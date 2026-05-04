// Ownership Rules
// 1. Each value in Rust has an owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.
fn main() {
    // Variable Scope
    {
        // s is not valid here, since it's not yet declared
        let s = "hello"; // s is valid from this point forward
        println!("{s}");

        // do stuff with s
    } // this scope is now over, and s is no longer valid

    {
        let mut s = String::from("Hello");
        s.push_str(", World!");
        println!("{s}");
    }

    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    } // this scope is now over, and s is no
      // longer valid

    // There is a natural point at which we can return the memory our String needs to the allocator:
    // when s goes out of scope. When a variable goes out of scope, Rust calls a special function
    // for us. This function is called drop, and it’s where the author of String can put the code to return
    // the memory. Rust calls drop automatically at the closing curly bracket.

    //   https://doc.rust-lang.org/stable/book/img/trpl04-03.svg

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
                   // x does NOT move into the function,
                   // so it's okay to use x afterward.

    // Here, x goes out of scope, then s. However, because s's value was moved,
    // nothing special happens.

    fn takes_ownership(some_string: String) {
        // some_string comes into scope
        println!("{some_string}");
    } // Here, some_string goes out of scope and `drop` is called. The backing
      // memory is freed.

    fn makes_copy(some_integer: i32) {
        // some_integer comes into scope
        println!("{some_integer}");
    } // Here, some_integer goes out of scope. Nothing special happens.

    // Rust does let us return multiple values using a tuple
    {
        let s1 = String::from("hello");

        let (s2, len) = calculate_length(s1);

        println!("The length of '{s2}' is {len}.");
    }

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }
}


//Rust has a feature for using a value without transferring ownership: references.