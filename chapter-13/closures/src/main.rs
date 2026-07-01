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
    // SYntax
    // || { code }

    // example 1
    let closure = || "Hello World!";

    println!("{}", closure());

    // example 2
    let add = |x: i32, y: i32| x + y;

    println!("ADD: {}", add(5, 6));

    //example 3 : not defining a parameter
    let add2 = |a, b| a + b;
    // println!("ADD2: {}", add2(10, 13));

    let hello: String = String::from("Hello");
    let world: String = String::from(", World!");

    println!("{}", add2(hello, &world));

    // capaturing cariables with closures
    // There are 3 different way to capture variables with closures:
    // 1. Borrowing a variable immutably
    // 2. Borrowing a variable mutably
    // 3. Taking ownership of a variable

    // example 4 - capturing by borrowing
    let x: i32 = 50;
    let print_x = || println!("{}", x);

    print_x();

    // examples 5 - capturing by mutable borrowing
    let mut y: i32 = 100;
    let mut print_y = || {
        y += 1;
        println!("{}", y);
    };
    print_y();

    // example 6 - capturing by taking ownership
    let z: String = String::from("Hello");

    let print_z = move || {
        println!("{}", z);
        drop(z);
    };
    print_z();

    // println!("z: {}", z);  this variable is no more available

    //closure traits
    // Fn: Captures variables by reference (&T)
    // FnMut: Captures variables by mutable reference (&mut T)
    // FnOnce: Captures variables by value (T)

    // example 7 closures as function parameters | applied here :-
    let double = |x| x * 2;
    apply(double);

    // example 8 - unction returning closures | applying it:
    let add_ten = create_adder(10);
    println!("{}", add_ten(5)); // output 15
    println!("{}", add_ten(100)); // output 110
}

// example7 - closures as function parameters
fn apply<F>(f: F)
where
    F: Fn(i32) -> i32,
{
    println!("{}", f(10)); //20;
}

/*
differences between Functions and Closures
1. capturing variables: closures can capture variables from the surrounding scope, whereas functions cannot.
2. syntax: closures are defined using the |args| body syntax, while funcions use the fn keyword.
3. flexibility:
   closures can be stored in variables, passed around as arguuments, and returned from other functions, giving them more flexiblity than traditional funcions.
4. Memory usages:
   closures that capture variables from theri environment may use more memory than regular funcions because they store those captured values.
*/

// example 8 - function returning closures
fn create_adder(increment: i32) -> impl Fn(i32) -> i32 {
    // We MUST use 'move' so the closure takes ownership of 'increment'
    move |x| x + increment
}