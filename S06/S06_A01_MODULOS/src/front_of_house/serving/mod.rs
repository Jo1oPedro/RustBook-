use crate::front_of_house::hosting::Breakfast;

pub fn dale() -> String {
    String::from("oi");
    let breakfast: Breakfast = super::hosting::Breakfast {
        toast: "dale".to_string(),
        seasonal_fruit: "cascata".to_string()
    };

    breakfast.seasonal_fruit
}