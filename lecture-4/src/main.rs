// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
//     color: Color,
// }

#[derive(Debug)]
enum Color {
    Yellow(String),
    Blue,
    Green,
    Red,
    White,
}

// #[derive(Debug)]
// enum IpAddrKind {
//     V4(String),
//     V6(String),
// }

// #[derive(Debug)]
// struct IpAddr {
//     kind: IpAddrKind,
//     address: String,
// }

// enum Option<T> {
//     Some(T),
//     None,
// }

fn main() {
    // let x = 5;
    // let y: Option<i8> = Some(3);

    // let sum = x + y.unwrap_or(0);
    // println!("Sum = {}", sum);

    // let rect = Rectangle {
    //     width: 3,
    //     height: 2,
    //     color: Color::Blue,
    // };

    // let ip_addr = IpAddr {
    //     kind: IpAddrKind::V4(String::from("127.0.0.1")),
    //     address: String::from("127.0.0.1"),
    // };

    // println!("ip_addr: {:?}", ip_addr);

    // get_color(Color::Red);

    // println!("{}", is_yellow(Some(Color::Blue)));

    // println!("The number is even = {}", is_even(32));

    is_yellow(None);
    // println!("Is color yellow? {:?}",);
}

// fn get_color(color: Color) {
//     match color {
//         Color::Yellow => {
//             println!("Color is yellow");
//         }
//         Color::Blue => {
//             println!("Color is blue");
//         }
//         _ => {
//             println!("Default case");
//         }
//     };
// }

// fn is_yellow(color: Color) -> bool {
//     match color {
//         Color::Yellow => true,
//         _ => false,
//     }
// }

// fn is_even(num: u32) -> bool {
//     match num % 2 {
//         0 => true,
//         _ => false,
//     }
// }

// fn is_yellow(color: Option<Color>) -> i32 {
//     match color {
//         Some(Color::Yellow) => 1,
//         Some(_) => 2,
//         None => 0,
//     }
// }

// switch(color) {
//     case Color.Yellow:
//         console.log("da")
//     default:
//         console.loig
// }

fn is_yellow(color: Option<Color>) {
    if let Some(_) = color {
    } else {
        println!("The value is None/null");
    }
}
