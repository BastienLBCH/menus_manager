use crate::controller::main_controller::RecipeSlot;
use crate::model::recipe::Recipe;

pub const MONDAY: &str = "Lundi";
pub const TUESDAY: &str = "Mardi";
pub const WEDNESDAY: &str = "Mercredi";
pub const THURSDAY: &str = "Jeudi";
pub const FRIDAY: &str = "Vendredi";
pub const SATURDAY: &str = "Samedi";
pub const SUNDAY: &str = "Dimanche";

#[derive(Clone, Debug)]
pub struct WeekDay {
    pub name: String,
    pub noon_recipe_slot: RecipeSlot,
    pub evening_recipe_slot: RecipeSlot,
    pub noon_recipe: Option<Recipe>,
    pub evening_recipe: Option<Recipe>,
}

impl WeekDay {
    pub fn new(
        name: String,
        noon_recipe_slot: RecipeSlot,
        evening_recipe_slot: RecipeSlot,
    ) -> WeekDay {
        WeekDay {
            name,
            noon_recipe_slot,
            evening_recipe_slot,
            noon_recipe: None,
            evening_recipe: None,
        }
    }
}
