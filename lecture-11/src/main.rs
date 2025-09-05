use std::{fs, process};

fn main() {
    // CLOSURE
    // let read_file = fs::read_to_string("poem.txt").unwrap_or_else(|err| {
    //     println!("Error");
    //     process::exit(1);
    // });

    // ITERATORS
    // let mut numbers = vec![1, 2, 3, 4];

    // let numbers_iter_1: Vec<i32> = numbers.iter().map(|x| x + 1).collect();
    // let number_iter_2: Vec<i32> = numbers.into_iter().filter(|x| x % 2 == 0).collect();
    // let number_iter_3 = numbers.iter_mut();

    // println!("numbers_iter_1: {:?}", number_iter_2);

    // match numbers.into_iter().reduce(|acc, item| acc + item) {
    //     Some(sum) => println!("sum: {}", sum),
    //     None => println!("None"),
    // }

    // let total = numbers.into_iter().reduce(|acc, item| acc + item);

    // match total {
    //     Some(sum) => println!("sum: {}", sum),
    //     None => println!("None"),
    // }

    // let mut value = 0;
    // for i in numbers {
    //     value += i;
    // }

    // println!("total: {}", total);

    // let city_1 = City {
    //     name: "Karachi".to_string(),
    //     population: 421,
    // };
    // let city_2 = City {
    //     name: "NYC".to_string(),
    //     population: 1,
    // };
    // let city_3 = City {
    //     name: "London".to_string(),
    //     population: 532234,
    // };

    // let mut cities: Vec<City> = vec![city_1, city_2, city_3];

    // cities.sort_by_key(|x| x.population);

    // nums.sort();

    // println!("nums: {:?}", nums);

    // for (index, city) in cities.iter().enumerate() {
    //     println!("city #{} - {}", index, city.name);
    // }

    let mut nums = [4, 3];

    // let mut nums_iter = nums.iter();

    // nums_iter.next();
    // nums_iter.next();
    // nums_iter.next();

    // println!("nums_iter: {:?}", nums_iter);
}

#[derive(Debug)]
struct City {
    name: String,
    population: u32,
}

// impl Iterator for City {
//     fn next()-> Option<<Self as Iterator>::Item> {
//         unimplemented!()
//     }
// }

// fn abc(fn1: fn)  {
//     return fn1
// }

// abc(abc())
