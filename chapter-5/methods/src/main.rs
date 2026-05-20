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

    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// Each struct is allowed to have multiple impl blocks

// The main reason for using methods instead of functions, in addition to providing method syntax
// and not having to repeat the type of self in every method’s signature, is for organization.

//  -> Operator
// Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic
// referencing and dereferencing. Calling methods is one of the few places in Rust with this behavior.

fn main() {
    println!("<- Methods ->");
    let rect1 = Rectangle {
        width: 50,
        height: 90,
    };

    let rect2 = Rectangle {
        width: 400,
        height: 70,
    };

    if rect2.width() {
        println!("The rectangle has a nonzero width; it is {}", rect2.width);
    }

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("can react1 hold react2? {}", rect1.can_hold(&rect2));
}
