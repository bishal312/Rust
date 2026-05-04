use std::io;

fn fibonacci_series_generator(num: u32) {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut c: u32;

    println!("Loop Start: ");
    for _ in 0..num {
        print!("{}, ", a);
        c = a + b;
        a = b;
        b = c;
    }
}

fn main() {
    let mut num = String::new();
    println!("Enter a number of length of a fibonacci series");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to readline.");

    let parsednum: u32;
    parsednum = num.trim().parse().expect("Failed to parse.");

    fibonacci_series_generator(parsednum);
}
