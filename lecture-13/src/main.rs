use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
enum ShapeKind {
    Square,
    Triangle,
    Rectangle,
}

// #[derive(Debug)]
// struct Shape {
//     name: ShapeKind,
//     next: Option<Box<Shape>>,
// }

// next => None Some(shape)

#[derive(Debug)]
enum Shape {
    Node(ShapeKind, Rc<RefCell<Shape>>),
    Nil,
}

fn main() {
    use Shape::*;

    // let a = String::from("Hello");
    // let b = &a;
    // let c = &a;

    // let num = 5;
    // let num = Box::new(5);

    // let shape_1 = Node(
    //     ShapeKind::Square,
    //     Box::new(Node(ShapeKind::Triangle, Box::new(Nil))),
    // );

    // loop {}

    // print_shape_name(&shape_1);

    // let a = &shape_1;
    // let c = &a;

    // println!("{:?}", shape_1);

    // let shape_2 = Shape {
    //     name: ShapeKind::Triangle,
    //     next: Some(Box::new(Shape {
    //         name: ShapeKind::Rectangle,
    //         next: None,
    //     })),
    // };

    // drop(&shape_2);

    // println!("shape: {:?}", shape_2);

    // REFCELL
    // let shape_1 = Rc::new(RefCell::new(Node(
    //     ShapeKind::Square,
    //     Rc::new(RefCell::new(Node(
    //         ShapeKind::Triangle,
    //         Rc::new(RefCell::new(Nil)),
    //     ))),
    // )));

    // let shape_2 = shape_1.borrow_mut();
    // println!("RC: {}", Rc::strong_count(&shape_1));

    // let shape_2 = Rc::new(Rc::clone(&shape_1));
    // println!("RC: {}", Rc::strong_count(&shape_1));
    // let shape_3 = Rc::new(Rc::clone(&shape_1));
    // println!("RC: {}", Rc::strong_count(&shape_1));

    // println!("shape 2: {:?}", shape_2);

    // let mut current = Rc::clone(&shape_1);

    // loop {
    //     let borrow = current.borrow();
    //     match &*borrow {
    //         Node(shape, next) => {
    //             println!("shape: {:?}", shape);

    //             let next_rc = Rc::clone(&next);
    //             drop(borrow);
    //             current = next_rc;
    //         }
    //         Nil => {
    //             println!("Shapes ended");
    //             break;
    //         }
    //     };
    // }

    // DEREF
    let a = String::from("Hadi");
    let b = Box::new(5);
    let c = b;
    let d = *c;

    // let mut a = 3;
    // a = 5;
}

// fn print_shape_name(node: &Shape) {
//     match node {
//         Shape::Node(shape, next) => {
//             println!("shape: {:?}", shape);
//             print_shape_name(next);
//         }
//         Shape::Nil => println!("Shapes ended"),
//     }
// }
