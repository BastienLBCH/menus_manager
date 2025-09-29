use crate::model::ingredient::Ingredient;
#[derive(Clone, Debug)]
pub struct Recipe {
    pub name: String,
    pub nbr_persons: u8,
    pub is_veggie: bool,
    pub ingredients: Vec<Ingredient>,
    pub steps: Vec<String>,
}

impl Recipe {
    pub fn new() -> Self {
        Recipe {
            name: "".to_string(),
            nbr_persons: 1,
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
}