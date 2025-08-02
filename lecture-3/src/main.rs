trait Shape {
    fn area(&self) -> u32;
    fn perimeter(&self) -> u32;
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Square {
    side: u32,
}

impl Shape for Rectangle {
    fn area(&self) -> u32 {
        return self.width * self.height;
    }

    fn perimeter(&self) -> u32 {
        return (self.width + self.height) * 2;
    }
}

impl Shape for Square {
    fn area(&self) -> u32 {
        return self.side * self.side;
    }

    fn perimeter(&self) -> u32 {
        return (self.side * 4);
    }
}

// struct Rectangle(i32, i32);

fn main() {
    let rect = Rectangle {
        width: 2,
        height: 3,
    };

    let square = Square { side: 3 };

    println!("{}", Rectangle::area(&rect));
    println!("{}", Square::area(&square));
    println!("{}", Square::perimeter(&square));
    // println!("{:?}", rect);

    // let area = area(Rectangle {
    //     width: 32,
    //     height: 31,
    // });
    // println!("{}", area);

    // let rect = Rectangle(31, 32);
    // println!("{}", rect.1);
}

// fn area(rect: Rectangle) -> i32 {
//     return rect.width * rect.height;
// }
