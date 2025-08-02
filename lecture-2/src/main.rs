fn main() {
    // let a = 5;

    // {
    //     let c = 6;
    //     println!("value of c = {}", c);
    // }

    // println!("value of c = {}", c);

    // let s1 = 5;
    // let s2 = s1;

    // println!("value of s2 = {}", s2);
    // println!("value of s1 = {}", s1);

    // let s3 = s1;

    let s1 = String::from("Hello World");
    // let s2 = s1.clone();

    // length(&s1);

    // println!("the value of s1 = {}", s1);

    // Dangling pointers and lifetimes
    // let reference_to_nothing = dangle();

    println!("val {}", &s1[3..]);
}

// fn dangle() -> String {
//     let s = String::from("hello");

//     s
// }

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn length(s: &String) -> usize {
//     s.capacity()
// }
