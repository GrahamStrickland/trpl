mod front_of_house;

mod customer {
    pub use crate::front_of_house::hosting;

    pub fn eat_at_restaurant() {
        // crate::front_of_house::hosting::add_to_waitlist();

        // front_of_house::hosting::add_to_waitlist();

        hosting::add_to_waitlist();
    }
}

fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order();
    }

    fn cook_order() {}
}
