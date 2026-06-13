fn main() {
    println!("Error Handling");

    // Rust groups errors into two major categories: recoverable and unrecoverable errors. For a recoverable error, such as a
    // file not found error, we most likely just want to report the problem to the user and retry the operation. Unrecoverable
    // errors are always symptoms of bugs, such as trying to access a location beyond the end of an array, and so we want to immediately stop the program.

    let v: Vec<i32> = vec![1, 2, 3];

    //unrecoverable error
    let res: i32 = v.get(90).copied().unwrap_or(0);
    println!("The value of res is {:?}", res);

    // panic!("This is my panic");
    // println!("This is end of the program");  // unreachable statement if panic!() is there

    recoverable_error();

    file_open_handle();
}

fn divide_nums(x: i32, y: i32) -> Result<i32, String> {
    // Result<T, E>
    if y == 0 {
        return Err(String::from("Please divide with non-zero number!"));
    }

    Ok(x / y)
}

fn recoverable_error() {
    println!("Recoverable_error Function!");

    // let result = divide_nums(10 / 2).unwrap_or(0);
    let result = match divide_nums(10, 0) {
        Ok(num) => num,
        Err(error) => {
            println!("Error:- {error}");
            -1
        }
    };
    println!("Result:- {:?}", result);
}

use std::fs::File;

fn file_open_handle() {
    let greeting_file_result = File::open("hello.txt");

    // let greeting_file: File = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("The file doesn't exist!"),
    // };

    //git check

    let greeting_file: File = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            std::io::ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("Problem creating the file: {e:?}"),
            },
            _ => panic!("Something went wrong: {error}"),
        },
    };
}
