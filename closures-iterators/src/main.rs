#![allow(unused_variables)]
#[allow(dead_code)]

use std::thread;
use std::time::Duration;
struct Catcher<T> where T: Fn(u32)->u32{
    calculation: T,
    value:Option<u32>
}

impl<T> Catcher<T> where T: Fn(u32)->u32{
    fn new (calculation:T) -> Catcher<T> {
        Catcher{
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg:u32)-> u32 {
        match self.value{
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                arg
            }

        }
    }
}

struct Shoe {
    size: u32,
    style: String,
}

struct Counter {
    count: u32
}
impl Counter {
    fn new() ->Counter{
        Counter{count:0}
    }
}
impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;
        if self.count < 6 {
            Some(self.count)
        }else{
            None
        }
    }
}
fn main() {
    println!("Hello, world!");
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    // let add_one_v3 = |x| { x + 1 };
    // let add_one_v4 = |x|               x + 1  ;
    // simulated_expensive_calculation(10);
    // let example_closure = |x| x;
    generate_workout(10, 45);
    // expensive_closure(5);

    let v1 = vec![1,2,3];
    let v1_it =v1.iter();

    for val in v1_it {
        println!("Got: {}", val);
    }

    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    println!("SUM: {}", total);

    let v1_it:Vec<_> = v1.iter().map(|x| x + 1).collect();
    for val in v1_it {
        println!("Got: {}", val);
    }
    let shoes = vec![
        Shoe { size: 10, style: String::from("sneaker") },
        Shoe { size: 13, style: String::from("sandal") },
        Shoe { size: 10, style: String::from("boot") },
    ];

    let in_my_size = shoes_in_my_size(shoes, 10);
    for shoe in in_my_size {
        println!("{}", shoe.style);
    }
    println!("Counter");
    let counter = Counter::new();
    for c in counter.zip(Counter::new().skip(1)) {
        println!("counter: {}, {}", c.0, c.1);
    }
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|x| x.size == shoe_size ).collect()
}


fn simulated_expensive_calculation(num: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    num
}

fn generate_workout(intensity: u32, random_number: u32) {
    // let expensive_closure = |num| {
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2));
    //     num
    // };
    // let expensive_result =
    //     simulated_expensive_calculation(intensity);

    let mut expensive_closure = Catcher::new(|num|{
        println!("calculating slowly...");
            thread::sleep(Duration::from_secs(2));
            num
    });
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            expensive_closure.value(intensity)
        );
        println!(
            "Next, do {} situps!",
            expensive_closure.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_closure.value(intensity)
            );
        }
    }
}