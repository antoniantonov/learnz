use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {};
    println!("I'm growing {plant:?}!");

    eat_at_restaurant();
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Added to waitlist");

            // From hosting module function we can access the parent module private function.
            super::super::front_of_house::serve_order();
        }
    }

    fn serve_order() {
        println!("Served order");
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // We cannot access a private function in the sibling module. The code below does not compile.
    // front_of_house::serve_order();

        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);
    
        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        // meal.seasonal_fruit = String::from("blueberries");

        // This compiles because all the variants are public. 
        let order1 = back_of_house::Appetizer::Soup;
}

mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }

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
