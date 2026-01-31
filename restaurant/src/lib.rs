pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn test() { println!("test"); }
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }
    fn cook_order() {}

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }
}

//use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    // // Absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // // Relative path
    // front_of_house::hosting::add_to_waitlist();

    hosting::add_to_waitlist();

    // Order a breakfast in the summer with Rye toast

    let mut meal = back_of_house::Breakfast::summer("Rye");
    println!("I'd like {} toast please", meal.toast);

    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;

}

mod customer {
    use crate::front_of_house::hosting;

    pub fn eat_at_restaurant2() {
        hosting::add_to_waitlist();
    }
}

use std::fmt;
use std::io;

fn function1() -> fmt::Result { Ok(()) }
fn function2() -> io::Result<()> { Ok(()) }


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
