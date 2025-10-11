use crate::model::ingredient::Ingredient;
use crate::model::weekday::WeekDay;

pub struct Menu {
    pub all_ingredients: Vec<Ingredient>,
    pub week_days: Vec<WeekDay>
}