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

fn add(num: i32, num2: Option<i32>) -> i32 {
    match num2 {
        Some(i) => num + i,
        None => num,
    }
}

fn plus_one(num: Option<i32>) -> Option<i32> {
    match num {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn dice_roll() {
    let dice_roll: i32 = 4;
    match dice_roll {
        3 => println!("You got a fancy hat🤠"),
        6 => println!("You lost a fancy hat😵"),
        other => println!("Move {} steps", other),
    }
}

fn main() {
    println!("Hello, world!");
    value_in_cent(Coin::Penny);
    println!("{:?}", add(50, Some(45)));
    println!("{:?}", plus_one(Some(50)));
    dice_roll();
}
