use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

fn main() {
    // let mut numbers = vec![1, 2, 3, 4, 5];
    // let numbers_2 = numbers.clone();

    // // {
    // //     let numbers_3 = numbers;
    // // }

    // let handler = thread::spawn(move || {
    //     // for mut i in numbers {
    //     //     i = i + 5;
    //     //     println!("hi number {i} from the spawned thread!");
    //     // }
    //     println!("numbers vector: {:?}", numbers);
    // });

    // println!("numbers vector: {:?}", numbers_2);

    // // for i in 1..100 {
    // //     println!("hi number {i} from the main thread!");
    // // }

    // handler.join().unwrap();

    // let (tx, rx) = mpsc::channel();

    // // tx.send(5);

    // // println!("message received from channel: {:?}", rx.recv());

    // let handler = thread::spawn(move || {
    //     for i in 0..10 {
    //         tx.send(i);
    //         // thread::sleep(Duration::from_millis(100));
    //     }
    // });

    // handler.join().unwrap();

    // for _ in 0..10 {
    //     println!("message received from channel: {:?}", rx.recv());
    // }

    // let mut mutex = Mutex::new(10);

    // thread::spawn(move || {
    //     let mut number = mutex.lock().unwrap();
    //     *number = 20;
    //     println!("number: {}", number);
    // });

    // let mut number = mutex.lock().unwrap();
    // println!("number in main function: {}", number);

    // thread::sleep(Duration::from_millis(1000))

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
