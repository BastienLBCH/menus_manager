pub const WHOLE_INGREDIENT: &str = "__WHOLE_INGREDIENT__";

#[derive(Clone, Debug)]
pub struct Ingredient {
    pub name: String,
    pub unit: String,
    pub quantity: f32,
}

impl Ingredient {
    pub fn to_string(&self) -> String {
        if self.unit == WHOLE_INGREDIENT {
            format!("{} : {}", self.quantity, self.name)
        } else {
            format!("{} : {} : {}", self.quantity, self.unit, self.name)
        }
    }
}
