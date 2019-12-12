use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];
    {
        let mut v = Vec::new();
        v.push(1);
        v.push(2);
        v.push(3);
    }

    println!("Hello, world!");

    let third = &v[2];
    // v.push(6);
    println!("third is: {}", third);

    match v.get(2) {
        Some(third) => println!("third is: {}", third),
        None => println!("None")
    }

    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for  i in &mut v {
        *i += 10;
        println!("{}", i);
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    impl SpreadsheetCell{
        fn value(&self) -> String {
            // String::from("sdgdsg")
            "sgdjkslgd".to_owned()
        }
    }

    for i in &row {
        println!("{} ", i.value());
    }
    for i in &row {
        // println!("{} ", i);
    }
    let mut s = String::new();
    let data = "initial contents";
    s = data.to_string();
    println!("{}",s);

    println!("{}",String::from("السلام عليكم"));
    println!("{}",String::from("Dobrý den"));
    println!("{}",String::from("Hello"));
    println!("{}",String::from("שָׁלוֹם"));
    println!("{}",String::from("नमस्ते"));
    println!("{}",String::from("こんにちは"));
    println!("{}",String::from("안녕하세요"));
    println!("{}",String::from("你好"));
    println!("{}",String::from("Olá"));
    println!("{}",String::from("Здравствуйте"));
    println!("{}",String::from("Hola"));

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("s: {}", s);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s1 is {}, s2 is {}", s1,s2);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; //s1 mooved
    println!("s1 is {}, s2 is {} s3 is{} ", "mooved", s2, s3);

    let hello = String::from("Здравствуйте");
    let answer = &hello[0..6];
    println!("answer is: {}", answer);
    let answer = hello.chars();

    for ch in answer{
        print!("{}-", ch);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.insert(String::from("Blue"), 30);

    println!("{:?}", scores);
    let key = String::from("Blue");
    let val = scores.get("Blue").unwrap();
    println!("{}", val );

    let mut number = HashMap::new();
    number.insert(1, "Um");
    number.insert(2, "Dois");
    number.insert(3, "Três");
    // number.insert(4, "Cinco");

    println!("{}", number.get(&1).unwrap());
    println!("{}", number.get(&2).unwrap());
    println!("{}", number.get(&3).unwrap());
    // println!("{}", number.get(&4).unwrap());
    let _4 = number.entry(6).or_insert("Seis");
    println!("{}", _4);
    println!("{}", number.entry(5).or_default());
    number.insert(4, "Quatro");
    println!("{}", number.get(&4).unwrap());
    println!("{}", number.get(&6).unwrap());
}
