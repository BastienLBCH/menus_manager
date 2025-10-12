use crate::model::ingredient::{Ingredient, WHOLE_INGREDIENT};
use crate::model::recipe::Recipe;
use crate::repository::recipe_repository::RecipeRepository;
use std::fs::File;
use std::io::Read;
use std::path::Path;

const RECIPE_DIRECTORY: &str = "recipes/";
const RECIPE_PART__NAME: &str = "name";
const RECIPE_PART__NBR_PERSONS: &str = "nbr_persons";
const RECIPE_PART__VEGGIE: &str = "veggie";
const RECIPE_PART__INGREDIENTS: &str = "ingredients";
const RECIPE_PART__STEPS: &str = "steps";

const ACCEPTED_BOOLEAN__TRUE: [&str; 6] = ["true", "yes", "oui", "y", "o", "t"];
const ACCEPTED_BOOLEAN__FALSE: [&str; 5] = ["false", "no", "non", "n", "f"];

pub struct RecipeService {
    pub recipe_repository: RecipeRepository,
}

impl RecipeService {
    pub fn new() -> RecipeService {
        RecipeService {
            recipe_repository: RecipeRepository::new(),
        }
    }

    pub fn gather_all_ingredients_from_recipes_vector(
        &self,
        all_recipes: &Vec<Recipe>,
    ) -> Vec<Ingredient> {
        let mut ingredients: Vec<Ingredient> = Vec::new();

        for recipe in all_recipes {
            let mut recipe = recipe.clone();
            recipe.sync_with_configured_nbr_persons();
            for ingredient_in_recipe in &recipe.ingredients {
                let ingredient_already_in_list_index = ingredients.iter().position(|ingr| {
                    ingr.name == ingredient_in_recipe.name && ingr.unit == ingredient_in_recipe.unit
                });

                if let Some(ingredient_index) = ingredient_already_in_list_index {
                    let mut already_existing_ingredient = ingredients[ingredient_index].clone();
                    ingredients.remove(ingredient_index);
                    already_existing_ingredient.quantity =
                        already_existing_ingredient.quantity + ingredient_in_recipe.quantity;
                    ingredients.push(already_existing_ingredient);
                } else {
                    ingredients.push(ingredient_in_recipe.clone());
                }
            }
        }

        ingredients
    }

    pub fn load_recipe(&mut self, recipe_file: &Path) -> Recipe {
        let mut file = File::open(recipe_file).unwrap();
        let mut content = String::new();
        file.read_to_string(&mut content).unwrap();
        let recipe_lines: Vec<&str> = content.split("\n").collect();

        let recipe_pattern = [
            RECIPE_PART__NAME,
            RECIPE_PART__NBR_PERSONS,
            RECIPE_PART__VEGGIE,
            RECIPE_PART__INGREDIENTS,
            RECIPE_PART__STEPS,
        ];
        let mut pattern_index = 0;
        let mut recipe = Recipe::new();
        let mut previous_turn_was_blank = false;
        for line_number in 0..recipe_lines.len() {
            let position_in_pattern = recipe_pattern[pattern_index];
            let current_line = recipe_lines[line_number].trim();

            if current_line == "" {
                if !previous_turn_was_blank {
                    pattern_index = pattern_index + 1;
                }
                previous_turn_was_blank = true;
                continue;
            } else if current_line.starts_with("#") {
                // The line is a comment
                continue;
            }
            previous_turn_was_blank = false;

            match position_in_pattern {
                RECIPE_PART__NAME => {
                    recipe.set_name(recipe_lines[line_number].to_string());
                }
                RECIPE_PART__NBR_PERSONS => {
                    let line_parts: Vec<&str> = current_line.split_whitespace().collect();
                    let nbr_persons_as_str = line_parts[line_parts.len() - 1].trim();
                    recipe.nbr_persons = nbr_persons_as_str.parse::<u8>().unwrap();
                    recipe.configured_nbr_persons = nbr_persons_as_str.parse::<u8>().unwrap();
                }
                RECIPE_PART__VEGGIE => {
                    let veggie_parts: Vec<&str> = current_line.split_whitespace().collect();
                    let str_boolean = veggie_parts[1].trim();
                    if ACCEPTED_BOOLEAN__TRUE.contains(&str_boolean) {
                        recipe.is_veggie = true;
                    } else if ACCEPTED_BOOLEAN__FALSE.contains(&str_boolean) {
                        recipe.is_veggie = false;
                    }
                }
                RECIPE_PART__INGREDIENTS => {
                    let ingredient_line_parts: Vec<&str> =
                        current_line.split(":").collect();

                    let mut ingredient_quantity: f32 = 0.0;
                    let mut ingredient_unit: String = String::from("");
                    let mut ingredient_name: String = String::from("");

                    match ingredient_line_parts.len() {
                        3 => {
                            ingredient_quantity = ingredient_line_parts[0].trim().parse::<f32>().unwrap();
                            ingredient_unit = ingredient_line_parts[1].trim().to_string();
                            ingredient_name = ingredient_line_parts[2].trim().to_string();
                        }
                        2 => {
                            ingredient_quantity = ingredient_line_parts[0].trim().parse::<f32>().unwrap();
                            ingredient_unit = WHOLE_INGREDIENT.to_string();
                            ingredient_name = ingredient_line_parts[1].trim().to_string();
                        }
                        _ => continue,
                    }

                    recipe.add_ingredient(Ingredient {
                        name: ingredient_name.to_string(),
                        unit: ingredient_unit.to_string(),
                        quantity: ingredient_quantity,
                    });
                }
                RECIPE_PART__STEPS => {
                    recipe.add_step(current_line.to_string());
                }
                _ => {
                    continue;
                }
            }
        }
        recipe
    }

    pub fn load_all_recipes(&mut self) {
        let paths = std::fs::read_dir(RECIPE_DIRECTORY).unwrap();
        for path in paths {
            let path = path.unwrap();
            if path.file_type().unwrap().is_file()
                && path.file_name().into_string().unwrap().ends_with(".txt")
            {
                let recipe = self.load_recipe(path.path().as_path());
                self.recipe_repository.add_recipe(recipe);
            }
        }
    }

    pub fn find_recipe_by_name(&self, recipe_name: &String) -> Recipe {
        self.recipe_repository.get_recipe(recipe_name)
    }

    pub fn list_recipes(&self, filter: String, only_veggies: bool) -> Vec<String> {
        let mut recipe_list: Vec<String> = Vec::new();
        match only_veggies {
            true => {
                let all_recipes_names = self.recipe_repository.list_all_recipes_names();
                for recipe_name in all_recipes_names {
                    if self.recipe_repository.get_recipe(&recipe_name).is_veggie {
                        recipe_list.push(recipe_name.to_string());
                    }
                }
            }
            false => recipe_list = self.recipe_repository.list_all_recipes_names(),
        }
        if filter != "" {
            recipe_list = recipe_list
                .into_iter()
                .filter(|recipe_name| {
                    recipe_name
                        .to_lowercase()
                        .starts_with(filter.to_lowercase().as_str())
                })
                .collect();
        }
        recipe_list
    }
}
