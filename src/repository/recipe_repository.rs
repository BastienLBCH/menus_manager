use std::collections::HashMap;
use crate::model::recipe::Recipe;

#[derive(Clone)]
pub struct RecipeRepository {
    recipes: HashMap<String, Recipe>,
}

impl RecipeRepository {
    pub fn new() -> RecipeRepository {
        RecipeRepository { recipes: HashMap::new() }
    }

    pub fn add_recipe(&mut self, recipe: Recipe) {
        self.recipes.insert(recipe.name.clone(), recipe);
    }

    pub fn list_all_recipes_names(&self) -> Vec<String> {
        self.recipes.keys().cloned().collect()
    }

    pub fn get_recipe(&self, name: &str) -> Recipe {
        self.recipes.get(name).unwrap().clone()
    }
}
