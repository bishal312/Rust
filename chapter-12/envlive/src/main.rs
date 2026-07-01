/*
Environment variables
They are a set of key-value paris that are stored in the operating system
and are used to store configuration settings and other information that is used
by the system and other applications.
*/

use dotenv::dotenv;
use std::env; // import env module // Import the dotenv module

fn main() {
    let key = "AAA";

    unsafe {
        env::set_var(key, "123");
    }

    // remove variable
    // env::remove_var(key);

    match env::var(key) {
        Ok(val) => println!("{}: {}", key, val),
        Err(e) => println!("Error {}: {}", key, e),
    }

    //example 2 - Read env variables from the command line interface(powershell)
    // let cli_arg = env::var("CLI_ARG");

    // match cli_arg {
    //     Ok(val) => println!("CLI_ARG: {:?}", val),
    //     Err(e) => println!("Error CLI_ARG: {}",e),
    // }

    // example 3 - read the env variables from a file
    dotenv().ok(); // dotenv sould be called at first of the code lines to load all variables and value.

    let api_key = env::var("API_KEY");

    match api_key {
        Ok(val) => println!("API_KEY: {:?}", val),
        Err(e) => eprintln!("Error_API_KEY: {}", e), // standard error instead of standard out (println)-> (eprintln)
    }

    println!("Program continious...");
}
