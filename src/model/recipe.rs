use crate::model::ingredient::Ingredient;


pub const NOON: &str = "Midi";
pub const EVENING: &str = "Soir";


#[derive(Clone, Debug)]
pub struct Recipe {
    pub name: String,
    pub nbr_persons: u8,
    pub configured_nbr_persons: u8,
    pub is_veggie: bool,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<String>,
}

fn round_to_2_digits(num: f32) -> f32 {
    (num * 100.0).round() / 100.0
}

impl Recipe {
    pub fn new() -> Self {
        Recipe {
            name: "".to_string(),
            nbr_persons: 1,
            configured_nbr_persons: 1,
            is_veggie: false,
            ingredients: Vec::new(),
            steps: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    pub fn add_ingredient(&mut self, ingredient: Ingredient) {
        self.ingredients.push(ingredient);
    }
    pub fn add_step(&mut self, step: String) {
        self.steps.push(step);
    }

    pub fn sync_with_configured_nbr_persons(&mut self) {
        for ingredient in self.ingredients.iter_mut() {
            let quantity_for_one_person = ingredient.quantity / self.nbr_persons as f32;
            let new_quantity = quantity_for_one_person * self.configured_nbr_persons as f32;
            ingredient.quantity = round_to_2_digits(new_quantity);
        }
    }
}