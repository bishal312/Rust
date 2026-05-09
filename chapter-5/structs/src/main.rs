fn _struct() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let user1 = User {
        active: true,
        username: String::from("bishal707"),
        email: String::from("bishal@gmail.com"),
        sign_in_count: 1,
    };

    // If the instance is mutable, we can change a value by using the dot notation and assigning into a particular field.
    // user1.email = String::from("mgr@gmail.com");

    // Build User
    fn build_user(email: String, username: String) -> User {
        user {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
}

// Creating Different Types with Tuple Structs
fn tuple_struct() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    // let Point(x, y, z) = origin; // destructure
}


fn main() {
    println!("<- struct function ->");
    _struct();
    tuple_struct();
}
