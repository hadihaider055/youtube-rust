//! This is lib.rs
//! We have two functions, search and read file

use std::{error::Error, fs, process};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

pub trait ConfigArgs {
    fn query(&self) -> String;
    fn file_path(&self) -> String;
}

impl Config {
    pub fn new<T: ConfigArgs>(args: &T) -> Config {
        let query = args.query();
        let file_path = args.file_path();

        Config {
            query: query,
            file_path: file_path,
        }
    }
}

pub fn search_query<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut output: Vec<&str> = Vec::new();

    for (i, line) in content.lines().enumerate() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            println!("Line {}: {}", i + 1, line);
            output.push(line);
        }
    }

    output
}

// fn read_file(file_path: &str) -> Result<(String), Box<dyn Error>> {
//     let content = fs::read_to_string(file_path)?;

//     Ok(content)
// }

pub fn read_file(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Failed to read file! {}", err);
        process::exit(1);
    });

    content
}

// pub fn read_file(file_path: &str) -> Result<String, String> {
//     if !file_path.ends_with(".txt") {
//         return Err("The file type must be of txt".to_string());
//     }

//     let content = fs::read_to_string(file_path).unwrap_or_else(|err| {
//         eprintln!("Failed to read file! {}", err);
//         process::exit(1);
//     });

//     Ok(content)
// }

// const arr = ["abc", "cde", "fgh"];
// arr.map((item, _id) => {
//     return (

//     )
// })
