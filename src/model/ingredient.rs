pub const WHOLE_INGREDIENT: &str = "__WHOLE_INGREDIENT__";

#[derive(Clone, Debug)]
pub struct Ingredient {
    pub name: String,
    pub unit: String,
    pub quantity: f32,
}
