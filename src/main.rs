use controller::main_controller::MainController;

mod model;
mod repository;
mod service;
mod controller;
mod view;

fn main() -> iced::Result {
    iced::run("Menus Manager", MainController::update, MainController::view)
}
