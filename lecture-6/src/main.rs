use std::{collections::HashMap, vec};

#[derive(Debug)]
enum Status {
    Todo(String),
    Inprogress(i32),
    Done,
}

fn main() {
    // let mut new_vec: Vec<i32> = Vec::new(); // [] -> i32
    // new_vec.push(3); // 0 index
    // new_vec.get(0);

    // let mut new_vec_2: Vec<i64> = vec![1, 2, 3];
    // new_vec_2[2] = 32312312123123123;
    // let abc_val: Option<&i64> = Some(&new_vec_2[2]);

    // for i in &mut new_vec_2 {
    //     *i *= 5;
    // }

    // println!("the vector value: {:?}", new_vec_2);

    // let vec_enum = vec![Status::Todo(String::from("Hello")), Status::Inprogress(32)];
    // println!("vector values: {:?}", vec_enum);

    // let abc: &str = "Hello";
    // let mut string_vec = String::from("Здравствуйте");
    // string_vec.push_str("string");
    // string_vec.get(0);

    // let ced: String = abc.to_string();
    // println!("{} {}", string_vec, abc);

    // for i in string_vec.chars() {
    //     println!("String val {}", i);
    // }

    // {
    //     name :string
    // }[]
    // [name: "Hadi", name: "Haider"]

    let mut hash_map_val: HashMap<String, String> = HashMap::new();
    hash_map_val.insert(String::from("name"), String::from("Hadi haider"));
    hash_map_val
        .entry(String::from("name"))
        .or_insert(String::from("Hadi "));

    println!("{:?}", hash_map_val.get("name"));
}
