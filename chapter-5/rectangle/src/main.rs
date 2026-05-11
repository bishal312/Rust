#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    println!("<- function starts! ->");

    let rect1 = (15, 30);

    let result = area(rect1);
    println!("The area of rectangle is : {:?}", result);

    // with struct
    let rect2 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("The area of rectangle2 is : {:?}", rect_area(&rect2));
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn rect_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// Note: Calling the dbg! macro prints to the standard error console stream
// (stderr), as opposed to println!, which prints to the standard output console stream (stdout). We’ll talk more about stderr and stdout
fn _dbg() {
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    dbg!(&rect1);
}


