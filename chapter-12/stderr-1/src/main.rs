use colored::*;
use std::fs::read_to_string;

//stdout and stderr

fn main() {
    let filename: &str = "non_existent_file.txt";

    if let Err(_e) = read_to_string(filename) {
        //print to stdout
        println!("This is printed on STDOUT");

        //print to stderr
        // eprintln!("This is printed on STDERR");

        // print in bold red color for eprintln
        eprintln!("{}", "This is printed on STDERR".bold().red());
    }
}
