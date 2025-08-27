use std::fs::File;
use std::io::{Error, ErrorKind, Read};

// enum Result<V, E> {
//     Ok(V),
//     Err(E)
// }

// #[derive(Debug)]
// struct Shape<T, V> { // Explaining Generics
//     width: T,
//     height: V,
// }

fn main() {
    // let rectangle = Shape {
    //     width: "32",
    //     height: 20,
    // };
    // println!("rectangle: {:?}", rectangle);

    println!("file: {:?}", read_username_from_file());

    // let arr = vec![1, 2, 3];
    // println!("value of index 2: {}", arr[6]);
    // print!("Hello1");
}

fn read_file(path: &str) -> Result<File, Error> {
    let read_file = File::open("example.txt");
    match read_file {
        Ok(content) => Ok(content),
        Err(error) => Err(error),
    }
    //   match read_file {
    //     Ok(content) => println!("file exists!"),
    //     Err(error) => panic!("File does not exist"),
    // };
}

fn read_username_from_file() -> Result<String, Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
