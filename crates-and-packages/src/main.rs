
mod front_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }
    impl Breakfast {
       pub fn create( toast: String) -> Breakfast{
            Breakfast{
                toast: toast,
                seasonal_fruit : String::from("seasonal_fruit")
            }
        }
    }
    fn print_message(strg: String){
        println!("{}", strg);
    }
   pub mod hosting {
        pub fn add_to_waitlist() {
            println!("add_to_waitlist");
            //super.print_message(String::from("add_to_waitlist"));
        }

        fn seat_at_table() {
            println!("seat_at_table");
        }
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

// use crate::back_of_house::Appetizer;
use back_of_house::Appetizer;
use front_of_house::hosting::add_to_waitlist;
use rand::Rng;
use std::collections::*;


// pub use crate::external_mod::message;

// pub mod external_mod;

// pub use crate::external_mod;

mod external_mod;

fn main() {
    println!("Hello, world!");
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();

    // let breakfast = front_of_house::Breakfast{
    //     toast: String::from("toast"),
    // };

    let breakfast = front_of_house::Breakfast::create(String::from("toast breakfast"));
    println!("breakfast: {}", breakfast.toast);

    // let order1 = back_of_house::Appetizer::Soup;
    // let order2 = back_of_house::Appetizer::Salad;

    let order1 = Appetizer::Soup;
    let order2 = Appetizer::Salad;
    add_to_waitlist();
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("secret_number: {}",secret_number);
    println!("external-mod: {}",external_mod::external_mod::message());

}
