use std::env::args;

use rust_cli::{Config, read_file, search_query};

fn main() {
    let mut args: Vec<String> = args().collect();

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
