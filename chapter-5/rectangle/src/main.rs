struct Rectangle {
    length: u32,
    height: u32,
}

fn main() {
    println!("<- function starts! ->");

    let rect1 = (15, 30);

    let result = area(rect1);
    println!("The area of rectangle is : {:?}", result);
}

fn area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}
