#[derive(Debug)]
enum UsState {
    Texas,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cent(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 6,
        Coin::Quarter(UsState::Texas) => {
            println!("The State is Texas");
            23
        }
        Coin::Quarter(State) => {
            println!("The State is {:?}", State);
            44
        }
    }
}

fn main() {
    println!("Hello, world!");
    value_in_cent(Coin::Quarter(UsState::Texas));
}
