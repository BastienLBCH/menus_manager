use crate::controller::main_controller::RecipeSlot;
use crate::model::ingredient::{Ingredient, WHOLE_INGREDIENT};
use crate::model::menu::Menu;
use crate::model::recipe::Recipe;
use crate::model::weekday::{
    FRIDAY, MONDAY, SATURDAY, SUNDAY, THURSDAY, TUESDAY, WEDNESDAY, WeekDay,
};
use rust_xlsxwriter::{Color, Format, Workbook, Worksheet};
use std::collections::HashMap;

#[derive(Debug)]
enum DailyRecipeSlot {
    NOON,
    EVENING,
}

const NOON: &str = "Midi";
const EVENING: &str = "Soir";

pub fn column_header_format() -> Format {
    let background_color = Color::RGB(0x32c1eb);
    Format::new()
        .set_background_color(background_color)
        .set_bold()
        .set_font_size(20)
}

fn write_shopping_list(workbook: &mut Workbook, menu: &Menu) {
    let mut worksheet = workbook.add_worksheet();
    let all_ingredients = menu.all_ingredients.clone();
    worksheet.set_name("Liste de courses").unwrap();

    let starting_row = 1;
    let starting_column = 1;

    let ingredient_name_column = starting_column + 0;
    let ingredient_quantity_column = starting_column + 1;
    let ingredient_unit_column = starting_column + 2;
    let week_resume_column = starting_column + 4;

    worksheet
        .write_with_format(
            starting_row,
            ingredient_name_column,
            "Ingrédient",
            &column_header_format(),
        )
        .unwrap();
    worksheet
        .write_with_format(
            starting_row,
            ingredient_quantity_column,
            "Quantité",
            &column_header_format(),
        )
        .unwrap();
    worksheet
        .write_with_format(
            starting_row,
            week_resume_column,
            "Résumé de la semaine",
            &column_header_format(),
        )
        .unwrap();

    for i in 0..all_ingredients.len() {
        let writing_row = starting_row + 1 + i as u32;
        let ingredient = all_ingredients[i].clone();
        worksheet
            .write(writing_row, ingredient_name_column, ingredient.name)
            .unwrap();
        worksheet
            .write(writing_row, ingredient_quantity_column, ingredient.quantity)
            .unwrap();
        if ingredient.unit != WHOLE_INGREDIENT {
            worksheet
                .write(writing_row, ingredient_unit_column, ingredient.unit)
                .unwrap();
        }
    }

    let mut writing_row = starting_row + 1;
    for i in 0..menu.week_days.len() {
        let week_day = &menu.week_days[i];
        worksheet
            .write(writing_row, week_resume_column, week_day.name.clone())
            .unwrap();

        writing_row = writing_row + 1;
        if let Some(recipe) = week_day.noon_recipe.clone() {
            worksheet
                .write(writing_row, week_resume_column, "Midi :")
                .unwrap();
            worksheet
                .write(writing_row, week_resume_column + 1, format!("{} ({} personnes)", recipe.name, recipe.configured_nbr_persons))
                .unwrap();
            writing_row = writing_row + 1;
        }

        if let Some(recipe) = week_day.evening_recipe.clone() {
            worksheet
                .write(writing_row, week_resume_column, "Soir :")
                .unwrap();
            worksheet
                .write(writing_row, week_resume_column + 1, format!("{} ({} personnes)", recipe.name, recipe.configured_nbr_persons))
                .unwrap();
            writing_row = writing_row + 1;
        }
        writing_row = writing_row + 1;
    }
}

fn write_recipe(
    worksheet: &mut Worksheet,
    week_day: WeekDay,
    daily_recipe_slot: DailyRecipeSlot,
    starting_row: u32,
    starting_column: u16,
) -> u32 {
    let mut daily_recipe_slot_name = "";
    let mut recipe_to_write: Option<Recipe>;

    match daily_recipe_slot {
        DailyRecipeSlot::NOON => {
            daily_recipe_slot_name = NOON;
            recipe_to_write = week_day.noon_recipe;
        }
        DailyRecipeSlot::EVENING => {
            daily_recipe_slot_name = EVENING;
            recipe_to_write = week_day.evening_recipe;
        }
    }

    if let Some(recipe) = recipe_to_write {
        worksheet
            .write_with_format(
                starting_row,
                starting_column,
                daily_recipe_slot_name,
                &column_header_format(),
            )
            .unwrap();
        worksheet
            .write_with_format(
                starting_row + 1,
                starting_column,
                recipe.name,
                &Format::new().set_bold(),
            )
            .unwrap();

        let mut current_row = starting_row + 2;

        for i in 0..recipe.ingredients.len() {
            worksheet
                .write(
                    current_row,
                    starting_column,
                    recipe.ingredients[i].clone().name,
                )
                .unwrap();
            worksheet
                .write(
                    current_row,
                    starting_column + 1,
                    recipe.ingredients[i].clone().quantity,
                )
                .unwrap();
            if recipe.ingredients[i].clone().unit != WHOLE_INGREDIENT {
                worksheet
                    .write(
                        current_row,
                        starting_column + 2,
                        recipe.ingredients[i].clone().unit,
                    )
                    .unwrap();
            }
            current_row = current_row + 1;
        }

        current_row += 2;

        for i in 0..recipe.steps.len() {
            worksheet
                .write(current_row, starting_column, recipe.steps[i].clone())
                .unwrap();
            current_row = current_row + 1;
        }
        current_row
    } else {
        starting_row
    }
}

fn write_day(workbook: &mut Workbook, week_day: WeekDay) {
    let mut worksheet = workbook.add_worksheet();
    worksheet.set_name(week_day.clone().name).unwrap();

    let mut starting_row = 1;
    let starting_column = 1;

    let last_written_row = write_recipe(
        &mut worksheet,
        week_day.clone(),
        DailyRecipeSlot::NOON,
        starting_row,
        starting_column,
    );
    starting_row = last_written_row + 3;
    let _ = write_recipe(
        &mut worksheet,
        week_day,
        DailyRecipeSlot::EVENING,
        starting_row,
        starting_column,
    );
}

pub fn write_excel_menu(menu: &Menu) {
    let mut workbook = Workbook::new();

    let recipes_slots_associated_to_week_days = HashMap::from([
        (RecipeSlot::MondayNoon, MONDAY),
        (RecipeSlot::MondayEvening, MONDAY),
        (RecipeSlot::TuesdayNoon, TUESDAY),
        (RecipeSlot::TuesdayEvening, TUESDAY),
        (RecipeSlot::WednesdayNoon, WEDNESDAY),
        (RecipeSlot::WednesdayEvening, WEDNESDAY),
        (RecipeSlot::ThursdayNoon, THURSDAY),
        (RecipeSlot::ThursdayEvening, THURSDAY),
        (RecipeSlot::FridayNoon, FRIDAY),
        (RecipeSlot::FridayEvening, FRIDAY),
        (RecipeSlot::SaturdayNoon, SATURDAY),
        (RecipeSlot::SaturdayEvening, SATURDAY),
        (RecipeSlot::SundayNoon, SUNDAY),
        (RecipeSlot::SundayEvening, SUNDAY),
    ]);

    write_shopping_list(&mut workbook, &menu);

    for day in menu.week_days.iter() {
        if day.clone().noon_recipe.is_some() || day.clone().evening_recipe.is_some() {
            write_day(&mut workbook, day.clone());
        }
    }

    workbook.save("menu.xlsx").expect("Write failed");
}
