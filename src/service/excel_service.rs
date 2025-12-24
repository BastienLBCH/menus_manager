use crate::controller::main_controller::RecipeSlot;
use crate::model::ingredient::{Ingredient, WHOLE_INGREDIENT};
use crate::model::menu::Menu;
use crate::model::recipe::Recipe;
use crate::model::recipe::{EVENING, NOON};
use crate::model::weekday::{
    FRIDAY, MONDAY, SATURDAY, SUNDAY, THURSDAY, TUESDAY, WEDNESDAY, WeekDay,
};
use crate::service::recipe_service::{RecipeParts, RecipeService};
use calamine::{Data, DataType, Reader, ToCellDeserializer, Xlsx, open_workbook};
use native_dialog::DialogBuilder;
use rust_xlsxwriter::{Color, Format, Workbook, Worksheet};
use std::collections::HashMap;
use std::fs::File;
use std::hash::Hash;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug)]
enum DailyRecipeSlot {
    NOON,
    EVENING,
}

pub fn column_header_format() -> Format {
    let background_color = Color::RGB(0x32c1eb);
    Format::new()
        .set_background_color(background_color)
        .set_bold()
        .set_font_size(20)
}

fn write_shopping_list(workbook: &mut Workbook, menu: &Menu) {
    let worksheet = workbook.add_worksheet();
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
                .write(writing_row, week_resume_column, format!("{NOON} :"))
                .unwrap();
            worksheet
                .write(
                    writing_row,
                    week_resume_column + 1,
                    format!(
                        "{} ({} personnes)",
                        recipe.name, recipe.configured_nbr_persons
                    ),
                )
                .unwrap();
            writing_row = writing_row + 1;
        }

        if let Some(recipe) = week_day.evening_recipe.clone() {
            worksheet
                .write(writing_row, week_resume_column, format!("{EVENING} :"))
                .unwrap();
            worksheet
                .write(
                    writing_row,
                    week_resume_column + 1,
                    format!(
                        "{} ({} personnes)",
                        recipe.name, recipe.configured_nbr_persons
                    ),
                )
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
    let daily_recipe_slot_name: &str;
    let recipe_to_write: Option<Recipe>;

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
        worksheet
            .write(
                starting_row + 1,
                starting_column + 1,
                recipe.configured_nbr_persons,
            )
            .unwrap();
        worksheet
            .write(starting_row + 1, starting_column + 2, "Personnes")
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

    write_shopping_list(&mut workbook, &menu);

    for day in menu.week_days.iter() {
        if day.clone().noon_recipe.is_some() || day.clone().evening_recipe.is_some() {
            write_day(&mut workbook, day.clone());
        }
    }

    workbook.save("menu.xlsx").expect("Write failed");
}

pub fn read_recipe_from_sheet_range(
    sheet_range: &calamine::Range<Data>,
    recipe_name_cell_coordinates: [usize; 2],
    recipe_configured_persons: Option<u8>,
) -> Option<Recipe> {
    let is_separation =
        |sheet_range: &calamine::Range<Data>, ceel_coordinates: [usize; 2]| -> bool {
            let first_cell = sheet_range
                .get(ceel_coordinates.into())
                .unwrap_or(&Data::Empty)
                .clone();
            let second_cell = sheet_range
                .get((ceel_coordinates[0] + 1, ceel_coordinates[1]))
                .unwrap_or(&Data::Empty)
                .clone();
            first_cell == second_cell && first_cell == Data::Empty
        };

    let recipe_pattern = [
        RecipeParts::Name,
        RecipeParts::Ingredients,
        RecipeParts::Steps,
    ];

    let recipe_name_row = recipe_name_cell_coordinates[0];
    let recipe_name_col = recipe_name_cell_coordinates[1];
    let mut extracted_recipe = {
        let mut recipe = Recipe::new();
        if let Some(recipe_configured_persons) = recipe_configured_persons {
            recipe.configured_nbr_persons = recipe_configured_persons;
        }
        recipe
    };

    let mut recipe_is_extracted = false;
    let mut position_in_pattern = 0;
    let mut i = 0;
    while !recipe_is_extracted {
        let recipe_part = &recipe_pattern[position_in_pattern];

        let cell_to_extract_coordinates = (recipe_name_row + i, recipe_name_col);

        let Some(extracted_data) = sheet_range.get(cell_to_extract_coordinates) else {
            recipe_is_extracted = true;
            break;
        };

        if is_separation(
            &sheet_range,
            [cell_to_extract_coordinates.0, cell_to_extract_coordinates.1],
        ) {
            if position_in_pattern == recipe_pattern.len() {
                recipe_is_extracted = true;
                break;
            }
            i += 2;
            position_in_pattern += 1;
            continue;
        }

        match recipe_part {
            RecipeParts::Name => {
                extracted_recipe.name = extracted_data.to_string();
                position_in_pattern += 1;
            }
            RecipeParts::Ingredients => {
                let ingredient_name = {
                    if extracted_data.clone() == Data::Empty {
                        break;
                    }
                    extracted_data.to_string()
                };
                let ingredient_quantity = {
                    let ingredient_quantity_cell = (
                        cell_to_extract_coordinates.0,
                        cell_to_extract_coordinates.1 + 1,
                    );
                    let cell_data = sheet_range.get(ingredient_quantity_cell).unwrap();
                    if cell_data.clone() == Data::Empty {
                        break;
                    }
                    cell_data.as_f64().unwrap() as f32
                };
                let ingredient_unit = {
                    let ingredient_unit_cell = (
                        cell_to_extract_coordinates.0,
                        cell_to_extract_coordinates.1 + 2,
                    );
                    let cell_data = sheet_range.get(ingredient_unit_cell).unwrap();
                    if cell_data.clone() == Data::Empty {
                        WHOLE_INGREDIENT.to_string()
                    } else {
                        cell_data.to_string()
                    }
                };

                let ingredient: Ingredient = Ingredient {
                    name: ingredient_name,
                    unit: ingredient_unit,
                    quantity: ingredient_quantity,
                };
                extracted_recipe.add_ingredient(ingredient);
            }
            RecipeParts::Steps => {
                let recipe_step = {
                    if extracted_data.clone() == Data::Empty {
                        break;
                    } else {
                        extracted_data.to_string()
                    }
                };
                extracted_recipe.add_step(recipe_step.to_string());
            }
            _ => {}
        }
        i += 1;
    }
    if recipe_is_extracted {
        Some(extracted_recipe)
    } else {
        None
    }
}

pub fn extract_recipe(
    recipe_service: &mut RecipeService,
    sheet_range: &calamine::Range<Data>,
    slot_name: &str,
) -> Option<Recipe> {
    let mut starting_cell: [usize; 2] = [9999, 9999];
    for (row, col, data) in sheet_range.cells() {
        let content_as_string = data.to_string();
        if content_as_string == slot_name.to_string() {
            starting_cell = [row, col];
            break;
        }
    }
    if starting_cell == [9999, 9999] {
        return None;
    }
    let recipe_name_row = starting_cell.clone()[0] + 1;
    let recipe_name_col = starting_cell.clone()[1];
    let recipe_name = sheet_range
        .get((recipe_name_row, recipe_name_col))
        .expect("Impossible to get recipe_name from sheet")
        .to_string();

    let recipe_configured_persons: Option<u8> = sheet_range
        .get((recipe_name_row, recipe_name_col + 1))
        .and_then(|value| value.as_i64().map(|i| i as u8));

    let recipe_in_recipe_service = recipe_service.find_recipe_by_name(&recipe_name);

    if let Some(recipe_configured_persons) = recipe_configured_persons
        && let Some(recipe) = recipe_in_recipe_service.clone()
    {
        let mut recipe_to_return = recipe.clone();
        recipe_to_return.configured_nbr_persons = recipe_configured_persons.clone();
        return Some(recipe_to_return);
    }

    if recipe_in_recipe_service.is_none() {
        let recipe: Option<Recipe> = read_recipe_from_sheet_range(
            sheet_range,
            [recipe_name_row, recipe_name_col],
            recipe_configured_persons,
        );
        if let Some(mut recipe) = recipe {
            if let Some(recipe_configured_persons) = recipe_configured_persons {
                recipe.configured_nbr_persons = recipe_configured_persons;
            }
            recipe_service.add_recipe(recipe.clone(), true);
            return Some(recipe);
        }
    }

    recipe_in_recipe_service
}

pub fn extract_data_from_sheet(
    recipe_service: &mut RecipeService,
    current_week_day: &WeekDay,
    sheet_range: &calamine::Range<Data>,
) -> WeekDay {
    let mut completed_week_day = current_week_day.clone();
    completed_week_day.noon_recipe = extract_recipe(recipe_service, sheet_range, NOON);
    completed_week_day.evening_recipe = extract_recipe(recipe_service, sheet_range, EVENING);

    completed_week_day
}

pub fn read_from_excel_menu(
    recipe_service: &mut RecipeService,
    week_days: Vec<WeekDay>,
) -> Option<Vec<WeekDay>> {
    let path = DialogBuilder::file().open_single_file().show().unwrap();

    let arranged_week_days: HashMap<String, WeekDay> = {
        let mut hashmap: HashMap<String, WeekDay> = HashMap::new();
        for week_day in week_days.clone() {
            hashmap.insert(week_day.clone().name, week_day.clone());
        }
        hashmap
    };

    let mut loaded_week_days: Vec<WeekDay> = Vec::new();

    let Some(path) = path else {
        return None
    };

    let path_as_str = path.to_str().unwrap();

    let mut workbook: Xlsx<_> = open_workbook(path.clone())
        .expect(format!("Failed to open workbook '{}'", path_as_str).as_str());
    let all_week_days_names_as_vector: Vec<String> =
        week_days.iter().map(|day| day.name.clone()).collect();
    let workbook_sheets: Vec<String> = workbook
        .sheet_names()
        .clone()
        .iter()
        .map(|sheet_name| sheet_name.to_string())
        .filter(|sheet_name| all_week_days_names_as_vector.contains(sheet_name))
        .collect();

    let arranged_sheets = {
        let mut hashmap: HashMap<String, calamine::Range<Data>> = HashMap::new();
        for sheet_name in workbook_sheets.clone() {
            hashmap.insert(
                sheet_name.clone(),
                workbook
                    .worksheet_range(sheet_name.as_str())
                    .expect(format!("Unable to extract data from sheet {sheet_name}").as_str()),
            );
        }
        hashmap
    };

    for sheet_name in workbook_sheets {
        let current_week_day = arranged_week_days
            .get(&sheet_name)
            .expect(format!("Failed to find week_day '{}'", sheet_name).as_str());
        let sheet_range = arranged_sheets.get(&sheet_name).expect(
            format!("Unable to extract sheet range from arranged_sheets {sheet_name}").as_str(),
        );
        loaded_week_days.push(extract_data_from_sheet(
            recipe_service,
            current_week_day,
            sheet_range,
        ))
    }
    if !loaded_week_days.is_empty() {
        return Some(loaded_week_days);
    }
    None
}
