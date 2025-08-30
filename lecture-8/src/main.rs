// struct Shape<T, C> {
//     width: T,
//     height: T,
//     a: C,
// }

struct Square {
    side: i32,
}

struct Rectangle {
    width: i32,
    height: i32,
}

trait Shape {
    fn area(&self) -> i32;
    fn perimeter(&self) -> i32;
}

impl Shape for Square {
    fn area(&self) -> i32 {
        self.side * self.side
    }

    fn perimeter(&self) -> i32 {
        (self.side) * 4
    }
}

impl Shape for Rectangle {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perimeter(&self) -> i32 {
        (self.height + self.width) * 2
    }
}

// fn area_square(dimensions: &Shape) -> i32 {
//     dimensions.width * dimensions.height
// }

// impl ShapeTrait for Shape {
//     fn area(&self) -> i32 {
//         self.width * self.height
//     }

//     fn perimeter(&self) -> i32 {
//         (self.height + self.width) * 2
//     }
// }

// impl ShapeTrait for Shape {
//      fn area(&self) -> i32 {
//         self.width * self.height
//     }

//     fn perimeter(&self) -> i32 {
//         (self.height + self.width) * 2
//     }
// }

fn main() {
    // let square = Shape {
    //     width: 5,
    //     height: 5,
    // };

    // let rectangle = Shape {

    // }

    // println!("{}", square.area());

    // let a: i32 = 32;

    // let rectangle = Shape {
    //     width: 32,
    //     height: 1,
    //     a: 32,
    // };

    // let b: T = 32;

    // let square = Square { side: 5 };

    // let rectangle = Rectangle {
    //     width: 5,
    //     height: 2,
    // };

    // println!(
    //     "area of square: {}, area of rectangle: {}",
    //     square.area(),
    //     rectangle.area()
    // );

    let a = 32;
    let mut result = 0;
    let mut c: i32;

    {
        let b = 1;
        result = greatest(&a, &b);
    }

    // result = a * c;
    println!("answer: {}", result)
}

fn greatest<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        return a;
    }

    b
}

// fn return_area(width: i32, height: i32) -> impl Shape {
//     // Square { side: width };

//     Rectangle {
//         width: width,
//         height: height,
//     }
// }

fn largest<T, U>(arr: Vec<T>, target: T) -> T
where
    T: PartialOrd + Copy,
{
    let mut result = target;

    for i in arr {
        if i > target {
            result = i
        } else {
            result = target
        }
    }

    return result;
}
