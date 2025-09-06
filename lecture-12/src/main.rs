//! This is my awesome crate
//!
//! Here goes some other description of what it is and what is does
//!
//! # Examples
//! ```
//! fn sum2(n1: i32, n2: i32) -> i32 {
//!   n1 + n2
//! }
//! # assert_eq!(4, sum2(2, 2));
//! ```

use cli_clap::{Config, ConfigArgs, read_file, search_query};

use clap::Parser;

/// CLI Arguments to Search for a Word in File
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Query (a word)
    #[arg(short, long)]
    query: String,

    /// File Path which will be considered as the default argument if nothing passed
    #[arg(short, long, default_value = "poem.txt")]
    file_path: String,
}

impl ConfigArgs for Args {
    fn query(&self) -> String {
        self.query.clone()
    }

    fn file_path(&self) -> String {
        self.file_path.clone()
    }
}

fn main() {
    let args = Args::parse();

    let config = Config::new(&args);

    let content = read_file(&config.file_path);
    search_query(&config.query, &content);
}

#[cfg(test)]
mod CLITesting {
    use super::*;

    #[test]
    fn test_search_feature() {
        let content = read_file("poem.txt");
        let searched_content = search_query("Hello", &content);

        assert_eq!(
            vec!["Hello world!", "Hello to the YT Community!"],
            searched_content
        )
    }

    #[test]
    fn test_read_file() {
        let content = read_file("poem.txt");

        assert_eq!(
            "Hello world!
Rust Lecture 10!
Hello to the YT Community!
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
",
            content
        )
    }
}

// WITH VALIDATION
// fn main() {
//     let args = Args::parse();

//     let config = Config::new(&args);

//     let content = read_file(&config.file_path);
//     search_query(&config.query, &content);
// }

// #[cfg(test)]
// mod CLITesting {
//     use super::*;

//     #[test]
//     fn test_read_file() {
//         let content = read_file("poem.txt");

//         assert_eq!(
//             "Hello world!
// Rust Lecture 10!
// Hello to the YT Community!
// I'm nobody! Who are you?
// Are you nobody, too?
// Then there's a pair of us - don't tell!
// They'd banish us, you know.

// How dreary to be somebody!
// How public, like a frog
// To tell your name the livelong day
// To an admiring bog!
// ",
//             content
//         )
//     }
// }
