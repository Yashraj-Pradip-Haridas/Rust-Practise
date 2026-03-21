// creating modules

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::add_to_waitlist();
}

fn serve_order_2() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();

        // super keyword allows us to  reference to parent module
        super::serve_order_2();
    }
    fn cook_order() {}
}

mod back_of_house_2 {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_2() {
    let mut meal = back_of_house_2::Breakfast::summer("Toast");

    // By default field within the struct are private
    meal.toast = String::from("Wheat");

    // But incase of an enum if you mark an enum public then all its fields are public
}

mod front_of_house_3;

pub use crate::front_of_house_3::hosting as fshosting; //using pub at start can allow external source to access the hosting too 
// use self::front_of_house_3::hosting; this can also used to refer to same crate
pub fn eat_at_restaurant_3() {
    fshosting::add_to_waitlist_3();
    // instead of using the following syntax we can use the above syntax by using the use keyword
    front_of_house_3::hosting::add_to_waitlist_3();
}

// the glob operator
use std::io::*; // brings all the public fns from the module to the code
