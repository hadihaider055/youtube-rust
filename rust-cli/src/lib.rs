use std::{fs, process};

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Config {
        if args.len() < 3 {
            eprintln!("You must have to pass the 2 arguments. Query, and the File Path");
            process::exit(1);
        }
        let query = &args[1];
        let file_path = &args[2];

        Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
        }
    }
}

pub fn search_query<'a>(query: &str, content: &'a str) -> Vec<&'a str> {
    let mut output: Vec<&str> = Vec::new();

    for line in content.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            println!("{}", line);
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

// const arr = ["abc", "cde", "fgh"];
// arr.map((item, _id) => {
//     return (

//     )
// })
