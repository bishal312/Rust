// Enums

// enum IpAddrKind {
//     v4,
//     v6,
// }

enum IpAddr {
    v4(String),
    v6(String),
}

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// let mypcip = IpAddr {
//     kind: IpAddrKind::v4,
//     address: String::from("127.0.0.1"),
// };

// let mypcip = IpAddr::v4(String::from("127.0.0.1"));

// let loopback = IpAddr {
//     kind: IpAddrKind::v6,
//     address: String::from("::1"),
// };

// let loopback = IpAddr::v6(String::from("::1"));

// Instance of enum

// let four = IpAddrKind::v4;
// let six = IpAddrKind::v6;

// fn route (ip_kind: IpAddrKind) {}

// struct Ipv4Addr {}

// struct Ipv6Addr {}

// enum IpAddr {
//     v4(Ipv4Addr),
//     v6(Ipv6Addr),
// }

// struct QuitMessage; // unit struct
// struct MoveMessage {
//     x: i32,
//     y: i32,
// }
// struct WriteMessage(String); // tuple struct
// struct ChangeColorMessage(i32, i32, i32); // tuple struct

// enum better than struct at this point

// enum Message {
//     Quite,
//     Move {x: i32, y: i32},
//     Write(String),
//     ChangeColor(i32, i32, i32),
// }

// There is one more similarity between enums and structs: Just as we’re able to define methods on structs using impl, we’re also able to define methods on enums. Here’s a method named call that we could define on our Message enum:

// impl Message {
//     fn call(&self) {
//         // method body would be defined here
//     }
// }

// Null alternative in rust
// enum Option<T> {
//     None,
//     Some(T),
// }

// let some_number = Some(5);
// let some_Char = Some('e');
// let absent_number: Option<i32> = None;

// let m = Message::Write(String::from("hello"));
// m.call();

fn lookup_Player(id: u32) -> Option<String> {
    if id == 1 {
        return Some("Bishal".to_string());
    }

    return None;
}

fn run_game() -> Option<( )> {
    // let player = match lookup_Player(1) {
    //     Some(p) => p,
    //     None => return
    // };
    let player = lookup_Player(1)?;
    println!("player: {}", player);
        Some(())
}


fn main() {
    println!("Enums and Option");

    run_game();
}
