/*
The Drop Trait

Drop lets you customize what happens when a value is about to go out of scope.
You can provide an implementation for the Drop trait on a type, and 
the code can be used to release resources when values of your type fo out of scope:
- files
- network connctions
- sockets

we are introduction it in the smart pointer section
because it is most commonly used with smart pointers.

1 - we define a smart pointer
2 - we implement the drop trait on the smart pointer
3 - we create an instance of the smart pointer in the main function
*/

use std::mem::drop;

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}
fn main() {
    let _c: CustomSmartPointer = CustomSmartPointer {
        data: String::from("my stuff"),
    };

    // c.drop() X
    drop(_c);

    let _d: CustomSmartPointer = CustomSmartPointer {
        data: String::from("other stuff"),
    };

    println!("CustomSmartPointer created.");
    
}
