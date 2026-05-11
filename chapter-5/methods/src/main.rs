// Methods are similar to functions: We declare them with the fn keyword and a name,
// they can have parameters and a return value, and they contain some code that’s run when
// the method is called from somewhere else.  Unlike functions, methods are defined within 
// the context of a struct (or an enum or a trait object

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    println!("<- Methods ->");
    let rect1 = Rectangle {
        width: 50,
        height: 90,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
