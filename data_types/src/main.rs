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

// Main Function
fn main() {
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


    //compound Data type
    // Tuple type
    let tup: (u32, &str, &str) = (100, "hi", "bsal"); // tuple is fixed and contain multiple type of data
    let (x, y, z) = tup;

    println!("{}", y);
    println!("{}", tup.0); //accessing items from tuple

    //Functions
    integer();
    floating_points();
}
