use std::error::Error;
use std::fs;
use std::env;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents: String =
        fs::read_to_string(config.file_path).expect("Something wend wrong while reading file");

    // println!("CONTENT:- {}", contents);
    // for line in search(&config.query, &contents) {
    //     if line.len() > 0 {
    //         println!("{}", line);
    //     }
    // }
    // Ok(())
    let results = search(&config.query, &contents);

    if results.is_empty() {
        println!("Query not found!");
    } else {
        for line in results {
            println!("{}", line);
        }
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {

    // pub fn build(args: &[String]) -> Result<Config, &'static str> {
    //     if args.len() < 3 {
    //         return Err("Not enough arguments");
    //     }

    //     let query = args[1].clone();
    //     let file_path = args[2].clone();

    //     Ok(Config { query, file_path })
    // }

    pub fn build(mut args: impl Iterator<Item = String>,) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

    // pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    //     let mut results = Vec::new();

    //     for line in contents.lines() {
    //         if line.contains(query) {
    //             // println!("{}", line);
    //             results.push(line);
    //         }
    //     }
    //     results
    // }
    
    pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        contents
            .lines()
            .filter(|line| line.contains(query))
            .collect()
    }

    pub fn search_case_insensative<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
        let query = query.to_lowercase();
        contents
            .lines()
            .filter(|line| line.to_lowercase().contains(&query))
            .collect()
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";
        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
