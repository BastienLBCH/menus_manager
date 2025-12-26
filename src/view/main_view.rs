use crate::controller::main_controller::Message::{
    DecrementedNbrPersonsOfRecipe, IncrementedNbrPersonsOfRecipe, ReturnButtonPressed,
};
use crate::controller::main_controller::{MainController, Message, RecipeSlot, View};
use crate::model::recipe::{Recipe, EVENING, NOON};
use crate::model::weekday::{FRIDAY, MONDAY, SATURDAY, SUNDAY, THURSDAY, TUESDAY, WEDNESDAY};
use iced::widget::{
    button, column, container, horizontal_rule, row, scrollable, text, text_input, toggler, vertical_rule,
    Button, Column, Row, Space, TextInput, Toggler,
};
use iced::{Alignment, Background, Color, Element, Length};

impl MainController {
    pub fn list_all_recipes_as_clickable_buttons(&self) -> Element<'_, Message> {
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
        let mut all_recipes_buttons: Column<Message> = Column::new();

        for recipe in all_recipes {
            all_recipes_buttons = all_recipes_buttons.push(button(text(recipe.clone())).on_press(
                Message::SelectedRecipe(self.slot_currently_in_edition.unwrap(), Some(recipe)),
            ));
        }
        all_recipes_buttons = all_recipes_buttons.push(Space::with_height(Length::Fixed(5.)));
        all_recipes_buttons = all_recipes_buttons.spacing(5);
        all_recipes_buttons.into()
    }

    pub fn view_recipe_selection(&self) -> Element<'_, Message> {
        let search_bar_content = {
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
            text_input("Filtrer par nom...", search_bar_content.as_str())
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
                    self.list_all_recipes_as_clickable_buttons(),
                ]
                .spacing(10),
            ]
            .spacing(10),
        )
        .into()
    }

    pub fn generate_recipe_selector(&self, recipe_slot: RecipeSlot) -> Element<'_, Message> {
        let button_name: String;
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
                select_recipe_button.height(Length::Fixed(75.)),
                container(
                    button("X")
                        .on_press(Message::SelectedRecipe(recipe_slot, None))
                        .style(|theme, status| button::Style {
                            background: Some(Background::Color(Color::from_rgb(
                                236. / 255.0,
                                8. / 255.0,
                                8. / 255.0
                            ))),
                            ..button::primary(theme, status)
                        })
                )
                .width(Length::Fill)
                .align_x(Alignment::End),
                row![
                    container(column![text(format!(
                        "Pour: {}",
                        selected_recipe.configured_nbr_persons.to_string()
                    )),])
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                    container(
                        column![
                            button("+")
                                .on_press(IncrementedNbrPersonsOfRecipe(recipe_slot, 1))
                                .width(Length::Fill)
                                .clip(false),
                            button("-")
                                .on_press(DecrementedNbrPersonsOfRecipe(recipe_slot, 1))
                                .width(Length::Fill)
                                .clip(false),
                        ]
                        .spacing(2)
                    )
                    .height(Length::Fill)
                    .align_y(Alignment::Center),
                ]
                .spacing(10)
            ]
            .spacing(20)
            .into()
        } else {
            select_recipe_button.into()
        }
    }

    pub fn generate_recipe_slot(
        &self,
        week_day: String,
        recipe_slot: RecipeSlot,
    ) -> Column<'_, Message> {
        column![
            text(week_day),
            Space::with_height(Length::Fixed(5.)),
            self.generate_recipe_selector(recipe_slot),
        ]
        .align_x(Alignment::Center)
    }

    pub fn generate_recipe_slots_row(
        &self,
        row_name: String,
        week_days: [&str; 7],
        recipe_slots: [RecipeSlot; 7],
    ) -> Row<'_, Message> {
        let mut recipe_slots_row = Row::new().spacing(12);
        recipe_slots_row = recipe_slots_row.push(
            column![text(row_name),]
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

    pub fn view_main(&self) -> Element<'_, Message> {
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
        let noon_row = self.generate_recipe_slots_row(String::from(NOON), week_days, noon_slots);
        let evening_row =
            self.generate_recipe_slots_row(String::from(EVENING), week_days, evening_slots);

        main_view = main_view.push(noon_row);
        main_view = main_view.push(horizontal_rule(2));
        main_view = main_view.push(evening_row);
        main_view = main_view.push(horizontal_rule(2));
        main_view = main_view.push(Space::with_height(Length::Fixed(10.0)));
        main_view = main_view.push(
            row![
                Space::with_width(Length::FillPortion(1)),
                button("Générer menu").on_press(Message::GenerateRecipeDocument),
                button("Importer").on_press(Message::ImportExcelFile),
                Space::with_width(Length::FillPortion(1))
            ]
            .spacing(10),
        );
        main_view = main_view.push(Space::with_height(Length::Fixed(10.0)));
        main_view.into()
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.current_view {
            View::Main => self.view_main(),
            View::RecipeSelection => self.view_recipe_selection(),
        }
    }
}
