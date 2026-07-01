// Minigrep is the cli which helps us with files utility.
// Command-Line Utility

use minigrep::Config;
use std::env;
use std::process;
// use std::{env, string};

// simple one
// fn main() {
//     let args: Vec<String> = env::args().collect();

//     // print the arguments
//     dbg!(&args);

//     // saving args in variable
//     let arg1: &String = &args[1];
//     let arg2: &String = &args[2];

//     //print the variables
//     println!("arg1: {}, arg2: {}", arg1, arg2);
// }

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        println!("Application error {}", e);
        process::exit(1);
    }
}

// fn parse_config(args: &[String]) -> Config {
//     let query: String = args[1].clone();
//     let file_path: String = args[2].clone();

//     Config{query, file_path}
// }
