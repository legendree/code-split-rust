mod back_of_house;
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_wailtist();
    hosting::add_to_wailtist();
    hosting::add_to_wailtist();
}
