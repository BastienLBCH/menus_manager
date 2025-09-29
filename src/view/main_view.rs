use crate::controller::main_controller::Message::ReturnButtonPressed;
use crate::controller::main_controller::{MainController, Message, RecipeSlot, View};
use iced::widget::{
    Button, Column, Space, TextInput, Toggler, button, column, horizontal_rule, pick_list, row,
    text, text_input, toggler, vertical_rule,
};
use iced::{Alignment, Element, Length};

impl MainController {
    pub fn generate_recipe_button(&self, slot: RecipeSlot) -> Button<Message> {
        let mut button_name = String::new();
        if self.selected_recipes.contains_key(&slot) {
            button_name = self.selected_recipes[&slot].name.clone()
        } else {
            button_name = "Sélectionnez une recette".to_string()
        }
        button(text(button_name))
            .on_press(Message::SelectedRecipeSlot(slot))
            .width(Length::Fill)
    }

    pub fn list_all_recipes__as_clickable_buttons(&self) -> Element<Message> {
        let only_veggie_recipes = self
            .slots_filtering_veggie_recipes
            .contains(&self.slot_currently_in_edition.unwrap());

        let filter = {
            if self
                .filters_on_recipes_slots
                .contains_key(&self.slot_currently_in_edition.unwrap())
            {
                self.filters_on_recipes_slots[&self.slot_currently_in_edition.unwrap()].clone()
            } else {
                "".to_string()
            }
        };

        let all_recipes = self
            .recipe_service
            .list_recipes(filter, only_veggie_recipes);
        let mut all_buttons: Column<Message> = Column::new();

        for recipe in all_recipes {
            all_buttons = all_buttons.push(button(text(recipe.clone())).on_press(Message::SelectedRecipe(self.slot_currently_in_edition.unwrap(), recipe)));
        }
        all_buttons = all_buttons.spacing(5);
        all_buttons.into()
    }

    pub fn view__recipe_selection(&self) -> Element<Message> {
        let search_bar__content = {
            if self
                .filters_on_recipes_slots
                .contains_key(&self.slot_currently_in_edition.unwrap())
            {
                self.filters_on_recipes_slots[&self.slot_currently_in_edition.unwrap()].clone()
            } else {
                "".to_string()
            }
        };
        let search_bar: TextInput<Message> =
            text_input("Filtrer par nom...", search_bar__content.as_str())
                .on_input(Message::FilteredSlotRecipe);

        let toggler_is_checked = self
            .slots_filtering_veggie_recipes
            .contains(&self.slot_currently_in_edition.unwrap());
        let veggie_toggler: Toggler<Message> = toggler(toggler_is_checked)
            .on_toggle(Message::FilteringVeggieRecipes)
            .label("Seulement les recettes végétariennes");

        row![
            button("Retour").on_press(ReturnButtonPressed),
            column![
                row![search_bar, veggie_toggler,].spacing(10),
                self.list_all_recipes__as_clickable_buttons(),
            ].spacing(10),
        ]
        .spacing(10)
        .into()
    }

    pub fn view__main(&self) -> Element<Message> {
        column![
            row![
                column![
                    Space::with_height(Length::FillPortion(1)),
                    text("Midi"),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .width(Length::Fixed(50.))
                .align_x(Alignment::Center),
                vertical_rule(2),
                // Space::with_width(Length::FillPortion(1)),
                column![
                    text("Lundi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::MondayNoon),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Mardi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::TuesdayNoon),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Mercredi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::WednesdayNoon),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Jeudi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::ThursdayNoon),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Vendredi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::FridayNoon),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Samedi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::SaturdayNoon),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Dimanche"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::SundayNoon),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                Space::with_width(Length::Fixed(0.)),
            ]
            .spacing(12),
            horizontal_rule(2),
            row![
                column![
                    Space::with_height(Length::FillPortion(1)),
                    text("Soir"),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .width(Length::Fixed(50.))
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Lundi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::MondayEvening),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Mardi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::TuesdayEvening),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Mercredi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::WednesdayEvening),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Jeudi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::ThursdayEvening),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Vendredi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::FridayEvening),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Samedi"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::SaturdayEvening),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                vertical_rule(2),
                column![
                    text("Dimanche"),
                    Space::with_height(Length::FillPortion(1)),
                    self.generate_recipe_button(RecipeSlot::SundayEvening),
                    Space::with_height(Length::FillPortion(1)),
                ]
                .align_x(Alignment::Center),
                Space::with_width(Length::Fixed(0.)),
            ]
            .spacing(12)
        ]
        .into()
    }

    pub fn view(&self) -> Element<Message> {
        match self.current_view {
            View::Main => self.view__main(),
            View::RecipeSelection => self.view__recipe_selection(),
        }
    }
}
