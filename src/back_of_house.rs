use crate::front_of_house::serving;

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

fn test_breakfast() {
    let a = Breakfast::summer("Whatever");

    let b = Breakfast {
        toast: String::from("Whatever"),
        seasonal_fruit: String::from("Whoever"),
    };
}

fn fix_incorrect_order() {
    cook_order();
    serving::serve_order();
}

fn cook_order() {}
