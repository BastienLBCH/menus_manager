use crate::controller::main_controller::Message::{
    DecrementedNbrPersonsOfRecipe, IncrementedNbrPersonsOfRecipe, ReturnButtonPressed,
};
use crate::controller::main_controller::{MainController, Message, RecipeSlot, View};
use crate::model::recipe::Recipe;
use crate::model::weekday::{FRIDAY, MONDAY, SATURDAY, SUNDAY, THURSDAY, TUESDAY, WEDNESDAY};
use iced::widget::{
    Button, Column, Row, Space, TextInput, Toggler, button, column, horizontal_rule, row,
    scrollable, text, text_input, toggler, vertical_rule,
};
use iced::{Alignment, Element, Length};

impl MainController {
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
            all_buttons = all_buttons.push(button(text(recipe.clone())).on_press(
                Message::SelectedRecipe(self.slot_currently_in_edition.unwrap(), Some(recipe)),
            ));
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

        scrollable(
            row![
                button("Retour").on_press(ReturnButtonPressed),
                column![
                    row![
                        search_bar,
                        veggie_toggler,
                        Space::with_width(Length::Fixed(10.0)),
                    ]
                    .spacing(10),
                    button(" -- AUCUNE RECETTE --").on_press(Message::SelectedRecipe(
                        self.slot_currently_in_edition.unwrap(),
                        None
                    )),
                    self.list_all_recipes__as_clickable_buttons(),
                ]
                .spacing(10),
            ]
            .spacing(10),
        )
        .into()
    }

    pub fn generate_recipe_selector(&self, recipe_slot: RecipeSlot) -> Element<Message> {
        let mut button_name = String::new();
        let mut selected_recipe: Option<Recipe> = None;
        if self.selected_recipes.contains_key(&recipe_slot) {
            selected_recipe = Some(self.selected_recipes[&recipe_slot].clone());
            button_name = self.selected_recipes[&recipe_slot].name.clone()
        } else {
            button_name = "Sélectionnez une recette".to_string()
        }

        let select_recipe_button: Button<Message> = Button::new(text(button_name))
            .on_press(Message::SelectedRecipeSlot(recipe_slot))
            .width(Length::Fill);

        if let Some(selected_recipe) = selected_recipe {
            column![
                select_recipe_button,
                row![
                    column![
                        Space::with_height(Length::Fill),
                        text(format!(
                            "Pour: {}",
                            selected_recipe.configured_nbr_persons.to_string()
                        )),
                        Space::with_height(Length::Fill),
                    ],
                    column![
                        button("+")
                            .on_press(IncrementedNbrPersonsOfRecipe(recipe_slot, 1))
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .clip(false),
                        button("-")
                            .on_press(DecrementedNbrPersonsOfRecipe(recipe_slot, 1))
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .clip(false),
                    ]
                    .spacing(2)
                ]
                .spacing(10)
            ]
            .spacing(20)
            .height(Length::Fill)
            .into()
        } else {
            select_recipe_button.into()
        }
    }

    pub fn generate_recipe_slot(
        &self,
        week_day: String,
        recipe_slot: RecipeSlot,
    ) -> Column<Message> {
        let mut spacing_height = 2;
        if self.selected_recipes.contains_key(&recipe_slot) {
            spacing_height = 1;
        }
        column![
            text(week_day),
            Space::with_height(Length::FillPortion(spacing_height)),
            self.generate_recipe_selector(recipe_slot),
            Space::with_height(Length::FillPortion(spacing_height)),
        ]
        .align_x(Alignment::Center)
    }

    pub fn generate_recipe_slots_row(
        &self,
        row_name: String,
        week_days: [&str; 7],
        recipe_slots: [RecipeSlot; 7],
    ) -> Row<Message> {
        let mut recipe_slots_row = Row::new().spacing(12);
        recipe_slots_row = recipe_slots_row.push(
            column![
                Space::with_height(Length::FillPortion(2)),
                text(row_name),
                Space::with_height(Length::FillPortion(2)),
            ]
            .width(Length::Fixed(50.))
            .align_x(Alignment::Center),
        );

        recipe_slots_row = recipe_slots_row.push(vertical_rule(2));

        for i in 0..7 {
            let week_day = week_days[i].to_string();
            let recipe_slot = recipe_slots[i].clone();
            recipe_slots_row =
                recipe_slots_row.push(self.generate_recipe_slot(week_day, recipe_slot));
            match i {
                6 => recipe_slots_row = recipe_slots_row.push(Space::with_width(Length::Fixed(0.))),
                _ => recipe_slots_row = recipe_slots_row.push(vertical_rule(2)),
            };
        }
        recipe_slots_row
    }

    pub fn view__main(&self) -> Element<Message> {
        let week_days: [&str; 7] = [
            MONDAY, TUESDAY, WEDNESDAY, THURSDAY, FRIDAY, SATURDAY, SUNDAY,
        ];
        let noon_slots: [RecipeSlot; 7] = [
            RecipeSlot::MondayNoon,
            RecipeSlot::TuesdayNoon,
            RecipeSlot::WednesdayNoon,
            RecipeSlot::ThursdayNoon,
            RecipeSlot::FridayNoon,
            RecipeSlot::SaturdayNoon,
            RecipeSlot::SundayNoon,
        ];

        let evening_slots: [RecipeSlot; 7] = [
            RecipeSlot::MondayEvening,
            RecipeSlot::TuesdayEvening,
            RecipeSlot::WednesdayEvening,
            RecipeSlot::ThursdayEvening,
            RecipeSlot::FridayEvening,
            RecipeSlot::SaturdayEvening,
            RecipeSlot::SundayEvening,
        ];

        let mut main_view = Column::new();
        let noon_row = self.generate_recipe_slots_row(String::from("Midi"), week_days, noon_slots);
        let evening_row =
            self.generate_recipe_slots_row(String::from("Soir"), week_days, evening_slots);

        main_view = main_view.push(noon_row);
        main_view = main_view.push(horizontal_rule(2));
        main_view = main_view.push(evening_row);
        main_view = main_view.push(horizontal_rule(2));
        main_view = main_view.push(Space::with_height(Length::Fixed(10.0)));
        main_view = main_view.push(row![
            Space::with_width(Length::FillPortion(1)),
            button("Générer menu").on_press(Message::GenerateRecipeDocument),
            button("Importer").on_press(Message::ImportExcelFile),
            Space::with_width(Length::FillPortion(1))
        ].spacing(10));
        main_view = main_view.push(Space::with_height(Length::Fixed(10.0)));
        main_view.into()
    }

    pub fn view(&self) -> Element<Message> {
        match self.current_view {
            View::Main => self.view__main(),
            View::RecipeSelection => self.view__recipe_selection(),
        }
    }
}
