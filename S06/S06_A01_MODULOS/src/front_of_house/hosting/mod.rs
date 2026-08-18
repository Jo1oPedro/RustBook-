pub struct Breakfast {
    pub toast: String,
    pub seasonal_fruit: String
}

pub enum Appetizer {
    Soup, 
    Salad
}

pub fn add_to_waitlist() -> String {
    String::from("add_to_waitlist")
}

fn take_payment() {
    println!("Função do módulo front_of_house: {}", super::init_front_of_house());
    println!("Função do submódulo hosting: {}", super::serving::dale());
    String::from("Take payment");
}