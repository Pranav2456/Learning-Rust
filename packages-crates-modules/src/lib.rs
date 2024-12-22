mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    } // Sibling module to serving defined within front_of_house

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    } // Sibling module to hosting defined within front_of_house
}

mod back_of_house {
    use crate::front_of_house::hosting;

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); //  super works like `..` in the filesystem. It allows us to reference an item that we know is a parent module.
    }

    fn cook_order() {}

    pub enum Appetizer {
        Soup,
        Salad,
    } // In contrast to structs, if we make an enum public, all of its variants are then public.
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    } // In structs, only the fields marked as public will be public. The rest of the struct will remain private.

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    // hosting::add_to_waitlist(); // This is an error because this shortcut no longer applies to the current scope.
}

pub use crate::front_of_house::hosting;
// Above is an example of a re-export. When we bring a name into scope with pub use, we're allowed to re-export that name to make it public. This technique is most commonly used to bring an item into scope and make it public, but also to rename items or to combine two items into one public item.
// Now there is no need to mark the hosting module as public, because we're re-exporting it as public in the crate root.
use crate::front_of_house::hosting::add_to_waitlist; // Idiomatic use paths

pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();

    // use keyword to shorten path
    hosting::add_to_waitlist();

    // idiomatic use paths
    add_to_waitlist();

    let mut meal = back_of_house::Breakfast::summer("Rye");

    meal.toast = String::from("Wheat");
    println!("I would like {} toast please", meal.toast);

    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}

fn deliver_order() {}