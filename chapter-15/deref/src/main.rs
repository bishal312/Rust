/*
The DEREF Trait
The Deref Trait allows you to customize the behavior of the dereference operator *.
The dereference operator is used to follow a reference to the value it points to.
By implementing so that the smart pointer can be treated like a regular reference,
you can write code that operates on references and use smart pointers instead.
The Dref trait is provided by the standard library.
*/

//Defining my own smart pointer
use std::ops::Deref;

impl <T> Deref for MyBox<T> {
    type Target = T; // Associated type, which is the type of the value that the type T is dereferencing to.

    fn deref(&self) -> &T {
        &self.0
    }
}

struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

// Deref Coercion
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let x: i32 = 5;
    let y: MyBox<i32> = MyBox::new(x);
    
    assert_eq!(5, x);
    assert_eq!(5, *y);
    let m: MyBox<String> = MyBox::new(String::from("Bishal"));

    hello(&m);
}

/*
Deref Coercion and Mutability
You can use the DerefMut trait to override the dereference operator on mutable references.
Rust does deref coercion when it finds types and trait implementations in the following cases:

- From &T to &U when T: Deref<Target=U>
- From &mut T to &mut U When T: DerefMut<Target=U>
- From &mut T to &U when T: Deref<Target=U>
*/
