fn deliver_order() {}

mod back_of_house {
    #[derive(Debug)]
    pub struct BreakFast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl BreakFast {
        pub fn summer(toast: &str) -> BreakFast {
            BreakFast {
                toast: String::from(toast),
                seasonal_fruit: String::from("Peaches"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order()
    }

    fn cook_order() {}
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("The functions called is add_to_waitlist");
        }

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

use crate::front_of_house::hosting;

mod customer {
    pub fn eat_at_restaurant() {
        // absolute path
        crate::front_of_house::hosting::add_to_waitlist();

        use crate::front_of_house::hosting;

        // relative path
        hosting::add_to_waitlist();

        // let breakfast = back_of_house::BreakFast::summer("Rye");
        // println!("You breakfast is {:?}", breakfast);
    }
}
pub fn eat_at_restaurant() {
    // absolute path
    // crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    hosting::add_to_waitlist();

    let breakfast = back_of_house::BreakFast::summer("Rye");
    println!("You breakfast is {:?}", breakfast);
}
