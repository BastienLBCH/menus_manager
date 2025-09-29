use crate::model::recipe::Recipe;
use crate::service::recipe_service::RecipeService;
use std::collections::HashMap;

pub struct MainController {
    pub recipe_service: RecipeService,
    pub selected_recipes: HashMap<RecipeSlot, Recipe>,
    pub slot_currently_in_edition: Option<RecipeSlot>,
    pub filters_on_recipes_slots: HashMap<RecipeSlot, String>,
    pub current_view: View,
    pub slots_filtering_veggie_recipes: Vec<RecipeSlot>,
}

impl Default for MainController {
    fn default() -> Self {
        let mut recipe_service = RecipeService::new();
        recipe_service.load_all_recipes();
        MainController {
            recipe_service: recipe_service,
            selected_recipes: HashMap::new(),
            filters_on_recipes_slots: HashMap::new(),
            slot_currently_in_edition: None,
            current_view: View::Main,
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
                    self.slots_filtering_veggie_recipes.push(self.slot_currently_in_edition.unwrap());
                } else {
                        if let Some(index) = self.slots_filtering_veggie_recipes.iter().position(|value| *value == self.slot_currently_in_edition.unwrap()) {
                            self.slots_filtering_veggie_recipes.swap_remove(index);
                        }
                }
            },
            Message::FilteredSlotRecipe(string) => {
                self.filters_on_recipes_slots
                    .insert(self.slot_currently_in_edition.unwrap(), string);
            }
        }
    }
}
