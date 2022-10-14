fn deliver_order() {}

mod front_of_house;

mod back_of_house {
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

    //all variants in enums are public by default, opposite to structs.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

//idiomatic use of paths
use crate::front_of_house::hosting;
//This exposes the "use" statement to the outside world
//pub use crate::front_of_house::hosting;
// use crate::front_of_house::hosting as hs;

//Convenient syntax
//use std::io::{self, Write};
pub fn eat_at_restaurant() {
    //crate refers to the root of the module tree--the start of the crate itself
    //full path
    // crate::front_of_house::hosting::add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    //relative path
    //  hs::add_to_waitlist();
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
