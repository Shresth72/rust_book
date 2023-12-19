// crate root

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}








fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}







// Privacy Rules
// 1. Modules cannot see into other modules
// 2. Structs, enums, and other items are private by default
// 3. Functions are public by default
// 4. You can use pub keyword to override default privacy rules

mod back_of_house_2 {
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
}

pub fn eat_at_restaurant_2() {
    let mut meal = back_of_house_2::Breakfast::summer("Wheat");

    meal.toast = String::from("Rye");
}







// Using Enums 
mod back_of_house_3 {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_3() {
    let _a = back_of_house_3::Appetizer::Soup;
    let _b = back_of_house_3::Appetizer::Salad;
}






use rand::{Rng, CryptoRng, ErrorKind::Transient};

use std::io::{self, Write};
use std::collections::*;

mod house;

// external crate
pub use self::house::hosting;

pub fn eat_at_restaurant_4() {
    let _secret_number = rand::thread_rng().gen_range(1, 101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
