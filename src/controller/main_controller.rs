use crate::model::ingredient::Ingredient;
use crate::model::menu::Menu;
use crate::model::recipe::Recipe;
use crate::model::weekday::{
    FRIDAY, MONDAY, SATURDAY, SUNDAY, THURSDAY, TUESDAY, WEDNESDAY, WeekDay,
};
use crate::service::excel_service::write_excel_menu;
use crate::service::recipe_service::RecipeService;
use std::collections::HashMap;

pub struct MainController {
    pub recipe_service: RecipeService,
    pub selected_recipes: HashMap<RecipeSlot, Recipe>,
    pub slot_currently_in_edition: Option<RecipeSlot>,
    pub filters_on_recipes_slots: HashMap<RecipeSlot, String>,
    pub current_view: View,
    pub week_days: Vec<WeekDay>,
    pub slots_filtering_veggie_recipes: Vec<RecipeSlot>,
}

impl Default for MainController {
    fn default() -> Self {
        let mut recipe_service = RecipeService::new();
        recipe_service.load_all_recipes();
        let week_days = Vec::from([
            WeekDay::new(
                String::from(MONDAY),
                RecipeSlot::MondayNoon,
                RecipeSlot::MondayEvening,
            ),
            WeekDay::new(
                String::from(TUESDAY),
                RecipeSlot::TuesdayNoon,
                RecipeSlot::TuesdayEvening,
            ),
            WeekDay::new(
                String::from(WEDNESDAY),
                RecipeSlot::WednesdayNoon,
                RecipeSlot::WednesdayEvening,
            ),
            WeekDay::new(
                String::from(THURSDAY),
                RecipeSlot::ThursdayNoon,
                RecipeSlot::ThursdayEvening,
            ),
            WeekDay::new(
                String::from(FRIDAY),
                RecipeSlot::FridayNoon,
                RecipeSlot::FridayEvening,
            ),
            WeekDay::new(
                String::from(SATURDAY),
                RecipeSlot::SaturdayNoon,
                RecipeSlot::SaturdayEvening,
            ),
            WeekDay::new(
                String::from(SUNDAY),
                RecipeSlot::SaturdayNoon,
                RecipeSlot::SundayEvening,
            ),
        ]);

        MainController {
            recipe_service: recipe_service,
            selected_recipes: HashMap::new(),
            filters_on_recipes_slots: HashMap::new(),
            slot_currently_in_edition: None,
            current_view: View::Main,
            week_days,
            slots_filtering_veggie_recipes: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
pub enum RecipeSlot {
    MondayNoon,
    MondayEvening,
    TuesdayNoon,
    TuesdayEvening,
    WednesdayNoon,
    WednesdayEvening,
    ThursdayNoon,
    ThursdayEvening,
    FridayNoon,
    FridayEvening,
    SaturdayNoon,
    SaturdayEvening,
    SundayNoon,
    SundayEvening,
}

#[derive(Debug, Clone)]
pub enum Message {
    SelectedRecipeSlot(RecipeSlot),
    ReturnButtonPressed,
    FilteredSlotRecipe(String),
    FilteringVeggieRecipes(bool),
    SelectedRecipe(RecipeSlot, Option<String>),
    GenerateRecipeDocument,
    IncrementedNbrPersonsOfRecipe(RecipeSlot, u8),
    DecrementedNbrPersonsOfRecipe(RecipeSlot, u8),
}

pub enum View {
    Main,
    RecipeSelection,
}

impl MainController {
    pub fn update(&mut self, message: Message) {
        match message {
            Message::SelectedRecipeSlot(slot) => {
                self.slot_currently_in_edition = Some(slot);
                self.current_view = View::RecipeSelection;
            }
            Message::ReturnButtonPressed => self.current_view = View::Main,
            Message::FilteringVeggieRecipes(is_filtering) => {
                if is_filtering {
                    self.slots_filtering_veggie_recipes
                        .push(self.slot_currently_in_edition.unwrap());
                } else {
                    if let Some(index) = self
                        .slots_filtering_veggie_recipes
                        .iter()
                        .position(|value| *value == self.slot_currently_in_edition.unwrap())
                    {
                        self.slots_filtering_veggie_recipes.swap_remove(index);
                    }
                }
            }
            Message::FilteredSlotRecipe(string) => {
                self.filters_on_recipes_slots
                    .insert(self.slot_currently_in_edition.unwrap(), string);
            }
            Message::SelectedRecipe(recipe_slot, recipe) => {
                if let Some(selected_recipe_name) = recipe {
                    let selected_recipe = self
                        .recipe_service
                        .find_recipe_by_name(&selected_recipe_name);
                    self.selected_recipes.insert(recipe_slot, selected_recipe);
                } else {
                    if self.selected_recipes.contains_key(&recipe_slot) {
                        self.selected_recipes.remove(&recipe_slot);
                    }
                }
                self.current_view = View::Main;
            }
            Message::IncrementedNbrPersonsOfRecipe(recipe_slot, nbr_persons) => {
                let mut recipe = self.selected_recipes.get(&recipe_slot).unwrap().clone();
                recipe.configured_nbr_persons = recipe.configured_nbr_persons + nbr_persons;
                self.selected_recipes.remove(&recipe_slot);
                self.selected_recipes.insert(recipe_slot, recipe.clone());
            }
            Message::DecrementedNbrPersonsOfRecipe(recipe_slot, nbr_persons) => {
                let mut recipe = self.selected_recipes.get(&recipe_slot).unwrap().clone();
                recipe.configured_nbr_persons = {
                    if recipe.configured_nbr_persons == 0 {
                        0u8
                    } else {
                        recipe.configured_nbr_persons - nbr_persons
                    }
                };
                self.selected_recipes.remove(&recipe_slot);
                self.selected_recipes.insert(recipe_slot, recipe.clone());
            }
            Message::GenerateRecipeDocument => {
                let mut week_days_to_print: Vec<WeekDay> = Vec::new();
                let noon_slots: [RecipeSlot; 7] = [
                    RecipeSlot::MondayNoon,
                    RecipeSlot::TuesdayNoon,
                    RecipeSlot::WednesdayNoon,
                    RecipeSlot::ThursdayNoon,
                    RecipeSlot::FridayNoon,
                    RecipeSlot::SaturdayNoon,
                    RecipeSlot::SundayNoon,
                ];

                for (recipe_slot, recipe) in self.selected_recipes.iter() {
                    let mut recipe = recipe.clone();
                    recipe.sync_with_configured_nbr_persons();
                    for week_day in self.week_days.iter_mut() {
                        let mut week_day = week_day.clone();
                        let mut should_write_day = false;

                        if week_day.noon_recipe_slot == recipe_slot.clone() {
                            week_day.noon_recipe = Some(recipe.clone());
                            should_write_day = true;
                        }

                        if week_day.evening_recipe_slot == recipe_slot.clone() {
                            week_day.evening_recipe = Some(recipe.clone());
                            should_write_day = true;
                        }

                        if should_write_day {
                            let day_index = week_days_to_print
                                .iter()
                                .position(|wd| wd.name == week_day.name);
                            match day_index {
                                Some(index) => {
                                    let mut new_week_day = week_days_to_print[index].clone();
                                    week_days_to_print.remove(index);
                                    if noon_slots.contains(recipe_slot) {
                                        new_week_day.noon_recipe = week_day.noon_recipe.clone();
                                    } else {
                                        new_week_day.evening_recipe =
                                            week_day.evening_recipe.clone();
                                    }
                                    week_days_to_print.push(new_week_day);
                                }
                                None => {
                                    week_days_to_print.push(week_day);
                                }
                            }
                        }
                    }
                }

                let all_recipes: Vec<Recipe> = self.selected_recipes.values().cloned().collect();
                let all_ingredients: Vec<Ingredient> = self
                    .recipe_service
                    .gather_all_ingredients_from_recipes_vector(&all_recipes);

                let menu = Menu {
                    all_ingredients,
                    week_days: week_days_to_print,
                };
                write_excel_menu(&menu);
            }
        }
    }
}
