use std::fs::File;
use std::io::{self, BufRead, BufReader};
// use std::io::{self, Read, BufRead, BufReader};


// Version 1
// fn main() -> io::Result<()> {
//     // open the file
//     let mut file: File = File::open("test.txt")?;

//     // initialize a string
//     let mut content: String = String::new();

//     // read the file content
//     file.read_to_string(&mut content)?;

//     //print the content on the console (opt)
//     println!("{}", content);

//     Ok(())
// }


// Version 2
// fn main() {
//     match read_file_to_string("test.txt") {
//         Ok(s) => println!("{}", s),
//         Err(e) => println!("Error: {}", e),
//     }
// }

// fn read_file_to_string(filename: &str) -> Result<String, io::Error> {
//     let mut file: File = File::open(filename)?;
//     let mut contents: String = String::new();

//     file.read_to_string(&mut contents)?;

//     Ok(contents)
// }

// version 3 line by line

fn main() -> io::Result<()> {
    let file: File = File::open("test.txt")?;

    //reader
    let reader: BufReader<File> = BufReader::new(file);

    // printer
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}