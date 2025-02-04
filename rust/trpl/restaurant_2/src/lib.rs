
// Define the module
mod front_of_house;

pub use crate::front_of_house::hosting;
pub use crate::front_of_house::hosting::add_to_waitlist as wl;

pub fn eat_at_restaurant() {

    // Multiple ways to invoke the function from the hosting module. 
    hosting::add_to_waitlist();
    wl();
}