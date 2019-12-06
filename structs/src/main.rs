fn main_old() {
    let mut user = User{
        username: String::from("Edurado"),
        email : String::from("eduardo.ssampaio@Outlook.com"),
        sign_in_count : 0,
        active: true
    };
    println!("Hello, world!");
    println!("User: {}", user.username );
    user.email = String::from("anotheremail@example.com");
    println!("user email: {}", user.email );

    let user2 =  build_user(String::from("User 2"),String::from("anotheremai2l@example.com"));
    println!("Use2r: {}", user2.username );
    println!("user2 email: {}", user2.email );

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("{} {} {} ", black.0, black.1, black.2);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,

    // fn area() -> u32 {
    //     width * height
    // }
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) ->Rectangle{
        Rectangle{width:size, height:size}
    }
}

// impl Rectangle {
//     fn can_hold(&self, other: &Rectangle) -> bool{
//         self.width > other.width && self.height > other.height
//     }
// }

// impl Rectangle{
//     fn square(size: u32) ->Rectangle{
//         Rectangle{width:size, height:size}
//     }
// }

fn main() {
    let rect1 = Rectangle {width: 30,height: 50};
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    let square = Rectangle::square(64);

    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!("The area of the rectangle is {} square pixels.",rect1.area());
    println!("The area of the rectangle is {} square pixels.",rect1.area());

    println!("Rect is {:?} ", rect1);

    println!("Rect1 hold rect2: {} ", rect1.can_hold(&rect2));
    println!("Rect1 hold rect3: {} ", rect1.can_hold(&rect3));

    println!("Square is: {:?} ", square);

}

fn area(ret: &Rectangle) -> u32{
    ret.width * ret.height
}