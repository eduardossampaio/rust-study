enum IpAddrKind {
    IPV4,
    IPV6
}



struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

enum IpAddrType2 {
    V4(u8, u8, u8, u8),
    V6(String),
}


fn main() {
    let four = IpAddrKind::IPV4;
    let six = IpAddrKind::IPV6;
    println!("Hello, world!");
    display_type(four);
    display_type(six);

    // if four == IpAddrKind::IPV4 {

    // }

    let home = IpAddr {
        kind: IpAddrKind::IPV4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::IPV6,
        address: String::from("::1"),
    };

    let home = IpAddrType2::V4(127,0,0,1);
    let loopback = IpAddrType2::V6(String::from("::1"));

    let some_number = Some(5);
    let some_string = Some(String::from("a string"));

    println!("some_number: {}, some_string: {}",some_number.unwrap(),some_string.unwrap());

    let x: i8 = 5;
    let y: Option<i8> = Some(5);

    let sum = x + y.unwrap();

    println!("Sum is: {}", sum);

    println!(" 1 + 1 : {}", plus_one( Some(1) ).unwrap() );
    println!(" 2 + 1 : {}", plus_one( Some(2) ) .unwrap());
    let none = plus_one(None);
    // println!(" none : {}", none.unwrap())// panic!

    let some_u8_value = Some(3);

    match some_u8_value {
        Some(3) => println!("three"),
        _ => println!("other"),
    }

    if let Some(3) = some_u8_value {
        println!("three");
    }

    println!("{}", some_u8_value.unwrap());

}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn display_type(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::IPV4 => println!("IPV4"),
        IpAddrKind::IPV6 => println!("IPV6")
    }
}