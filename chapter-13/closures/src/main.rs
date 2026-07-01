/*
Closure in Rust

Closures are blcok of code that can be : 
- Stroed in variables
- Passed as arguments to functions
- returned from functions

They are similar to functions but with one main difference:
They can capture variables from theri surrounding scope.

Key features:
- **Anonymous Functions**:
Closures are unnamed functions that can be stored in varibles or passed to other functions.

- **Capturing Environment**:
Closures can capture values from their surrounding scope by borrowing, mutably borrowing, or taking ownership of them.

- **Type Interence**;
Rust infers the types of parameters and return types in most closures, so explicit type annotations are often unnecessary.

- **Flexibility**:
Closures can be stored as function pointers or as traits like "Fn", "FnMut", and "FnOnce", depending on how they capture variables..
 */

fn main() {
    println!("Hello, world!");
}
