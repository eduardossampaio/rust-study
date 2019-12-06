fn main() {
    const MAX_POINTS: u64 = 100_0001_00001_00001;
    println!("Hello, world!");
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {} const is {}", x, MAX_POINTS);

    let x = 5;
    let x = x + 1;
    let x = x * 2;

    let byte : u8 = 0b10101101;

    let tup: (i32, f64, u8) = (500, 6.4, 1);

    println!("The value of x is: {}, byte is: {}", x, byte);

    let (tup1,tup2,tup3) = tup;
    println!("Tupla: {} {} {} ", tup1,tup2, tup3);
    println!("Tupla: {} {} {} ", tup.0,tup.1, tup.2);
    another_function(123);
    print_tuple(tup);
    print_tuple(tup);

    let y = {
        let x = 3;
        x + 1
    };

    println!("Y  is : {}", y);
    println!("five() : {}", five());

    let condition = false;
    let number = if condition {
        5
    } else {
        6
    };

    println!("The value of number is: {}", number);

    let mut number = 0;
    while number <10 {
        number += 1;
    }

    println!("Number is: {}", number);

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("element is: {}", element);
    }

    for number in (2..4).rev() {
        println!("NUm is: {}", number);
    }

    let mut t = String::from("Hello");
    t = t + ", World";
    println!("{}",t);

    let t2 = t.clone();
    println!("{} {}",t,t2);

    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let mut str_test =String::from("Dummy string");
    str_test = prints_string(str_test);
    prints_string(str_test);


    let mut test_str = String::from("Hello");
    let test_str_len = calculate_length(&test_str);
    println!("string is {} len : {}", test_str, test_str_len);

    if test_str == "Hello"  {
        println!("IGUAL");
    }else{
        println!("DIFERENTE");
    }

    change(&mut test_str);
    println!("string is {}",test_str);

    let r1 = &test_str; // no problem
    let r2 = &test_str; // no problem
    println!("{} and {}", r1, r2);
    let r3 = &mut test_str; // no problem
    println!("{}", r3);

    let s = String::from("hello world");

    let hello = &s[0..5];
    let world = &s[6..11];

    println!("hello {} world {}", hello, world);

}

fn prints_string(strs: String) -> String{
    println!("{}",strs);
    strs
}
fn five() -> i32 {
    5
}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_tuple(tup: (i32, f64, u8) ){
    println!("Tupla: {} {} {} ", tup.0,tup.1, tup.2);
}

fn calculate_length( s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}