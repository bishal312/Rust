// Length	Signed	Unsigned
// 8-bit	i8	    u8
// 16-bit	i16	    u16
// 32-bit	i32	    u32
// 64-bit	i64	    u64
// 128-bit	i128	u128
// Architecture-dependent	isize	usize

// Scalar type
// Integer
fn integer() {
    let a: i32 = "15".parse().expect("Not a number"); // signed
    let b: i64 = "500".parse().expect("Not a number"); // signed

    let c: u32 = 777; // unsigned
    let d: u32 = 888; // unsigned

    println!("{}", a);
    println!("{}", b);
    println!("{}", c);
    println!("{}", d);
}

// Floating-points
fn floating_points() {
    let x = 6.0; // f64 --> default
    let y: f32 = 3.0; // f32

    let a: f64 = "56".parse().expect("No no no, it's not a number");

    println!("{}", x);
    println!("{}", y);
    println!("{}", a);
}

// Compound type
fn compound_type() {
    //compound Data type
    // Tuple type
    let tup: (u32, &str, &str) = (100, "hi", "bsal"); // tuple is fixed and contain multiple type of data
    let (x, y, z) = tup;

    println!("{}", y);
    println!("{}", tup.0); //accessing items from tuple

    // Array type
    let arr = [1, 2, 3, 4, 5];
    let arr2 = [2; 4];

    println!("{arr2:?}"); // method 1 debug formatting
    println!("{:?}", arr2); // method 2

    println!("{}", arr2[3]); // access
}

//Normal operations
fn simple_operations() {
     // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;

    let t = true;

    let f: bool = false; // with explicit type annotation

    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

use std::io;

// Invalid Array Elements access
fn invalid_arr_access() {
    let a = [1, 2, 3, 4, 5];

    println!("Please Enter an array index?");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number.");

    let Element = a[index];

    println!("The value of the element at index {index} is : {Element}");    
}

// Main Function
fn main() {
   

    //Functions
    integer();
    floating_points();
    compound_type();
    simple_operations();
    invalid_arr_access();
}
