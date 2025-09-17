use serde::Deserialize;
use serde_json::from_str;
use std::future::Future;
use std::io::{self, stdin};
use std::time::Duration;
use trpl::StreamExt;
use trpl::{Either, Html};

fn main() {
    // get the input from user and print the square of the input

    // loop {
    //     let mut user_input = String::new();

    //     println!("Enter a number to square");
    //     io::stdin()
    //         .read_line(&mut user_input)
    //         .expect("failed to read input");

    //     let number = user_input
    //         .trim()
    //         .parse::<u32>()
    //         .expect("Failed to parse the string");

    //     println!("Square of the number {user_input} = {}", number * number);
    // }

    // stdin()
    //     .lines()
    //     .map(|x| x.ok())
    //     .map(|y| {
    //         y.and_then(|z| z.trim().parse::<u32>().ok())
    //             .and_then(|val| {
    //                 let square = val * val;

    //                 println!("Square of the number {val} = {}", square);

    //                 Some(())
    //             })
    //     })
    //     .for_each(|_| ());

    // println!("hello1");
    // trpl::run(get_placeholder_data());
    // println!("hello2");

    // trpl::run(async {
    //     let url = "https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html";
    //     match page_title(url).await {
    //         Some(title) => println!("The title for {url} was {title}"),
    //         None => println!("{url} had no title"),
    //     }
    // })

    // trpl::run(async {
    //     let title_fut_1 = page_title("https://hadi-haider.netlify.app");
    //     let title_fut_2 = page_title("https://hadihaider055.github.io");

    //     let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
    //         Either::Left(left) => left,
    //         Either::Right(right) => right,
    //     };

    //     println!("{url} returned first");
    //     match maybe_title {
    //         Some(title) => println!("Its page title is: '{title}'"),
    //         None => println!("Its title could not be parsed."),
    //     }
    // })

    // trpl::run(async {
    //     trpl::spawn_task(async {
    //         for i in 1..10 {
    //             println!("hi number {i} from the first task!");
    //             trpl::sleep(Duration::from_millis(500)).await;
    //         }
    //     });

    //     for i in 1..5 {
    //         println!("hi number {i} from the second task!");
    //         trpl::sleep(Duration::from_millis(500)).await;
    //     }
    // });

    // trpl::race()

    // trpl::join_all(iter)

    trpl::run(async {
        let values = 1..101;
        let iter = values.map(|n| n * 2);
        let stream = trpl::stream_from_iter(iter);

        let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

        while let Some(value) = filtered.next().await {
            println!("The value was: {value}");
        }
    });
}

// #[derive(Debug, Deserialize)]
// struct Post {
//     id: u32,
//     userId: u32,
//     title: String,
//     body: String,
// }
// async fn get_placeholder_data() {
//     let response = trpl::get("https://jsonplaceholder.typicode.com/posts?_limit=5").await;

//     let body = response.text().await;
//     let posts: Vec<Post> = from_str(&body).expect("failed to parse string");

//     println!("{:?}", posts);
// }

// fn page_title(url: &str) -> impl Future<Output = Option<String>> {
//     async move {
//         let text = trpl::get(url).await.text().await;
//         Html::parse(&text)
//             .select_first("title")
//             .map(|title| title.inner_html())
//     }
// }

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;
    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}

// const num_func = async () => {
//     const response = await fetch(url);
//     return response.data;

// }

// fn main() {
//     num_func();

//  let a  = 5;
//  console.log(a);
// let b = 1;
// console.log(b);

// }
