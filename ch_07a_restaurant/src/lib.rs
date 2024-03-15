mod front_of_house {
    pub mod hosting {
        pub fn add_to_wait_list() {}
    }
}

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_wait_list();
    front_of_house::hosting::add_to_wait_list();
}
