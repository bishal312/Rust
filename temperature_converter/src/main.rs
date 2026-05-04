use std::io;

fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

fn celsius_to_fahrenheit(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

fn main() {
    println!("Temperature Converter");

    let mut selector = String::new();
    let mut temperature_input = String::new();

    loop {
        selector.clear();
        temperature_input.clear();

        println!("Enter f for fahrenheit_to_celsius and c for celsius_to_fahrenheit conversion: ");
        io::stdin()
            .read_line(&mut selector)
            .expect("Failed to read line.");

        match selector.trim() {
            "f" => {
                println!("Enter temperature in Fahrenheit: ");
                io::stdin()
                    .read_line(&mut temperature_input)
                    .expect("Failed to read line.");

                if let Ok(temp) = temperature_input.trim().parse::<i32>() {
                    let result = fahrenheit_to_celsius(temp);
                    println!("{} F = {} C", temp, result);
                    println!("You can enter E to exit.")
                } else {
                    println!("Invalid temperature.");
                    println!("You can enter E to exit.")
                }
            }
            "c" => {
                println!("Enter temperature in Celsius: ");
                io::stdin()
                    .read_line(&mut temperature_input)
                    .expect("Failed to read line.");

                if let Ok(temp) = temperature_input.trim().parse::<i32>() {
                    let result = celsius_to_fahrenheit(temp);
                    println!("{} C = {} F", temp, result);
                    println!("You can enter E to exit.")
                } else {
                    println!("Invalid temperature.");
                    println!("You can enter E to exit.")
                }
            }
            "e" => {
                break;
            }
            _ => {
                println!("Please enter 'f' or 'c'.");
                println!("You can enter 'e' to exit.")
            }
        }
    }
}
