fn main() {
    println!("Hello, world!");
    another_function();

    let x = eight();
    println!("{}", x);

    let y = plus_one(5);

    println!("The value of y is: {y}");
}

fn another_function() {
    println!("another function!");

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");
    // Note the x + 1 line without a semicolon at the end, which is unlike most
    // of the lines you’ve seen so far. Expressions do not include ending semicolons.
    // If you add a semicolon to the end of an expression, you turn it into a statement,
    // and it will then not return a value. Keep this in mind as you explore function
    // return values and expressions next.
}

fn eight() -> i32 {
    8
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
