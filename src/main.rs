mod menu;
mod structures;
mod services;
mod enums;
mod tools;
use crate::enums::main_menu_user_choice::MainMenuUserChoice;
use crate::structures::user::User;

fn match_input(input: u8, users: &mut Vec<User>) -> bool {
    let chosen_input = MainMenuUserChoice::from_value(input);
    match chosen_input {
        MainMenuUserChoice::AddUser => {
            let user_creation_input = menu::user::get_user_creation_input();
            services::user::add_user(users, user_creation_input);
        }
        MainMenuUserChoice::RemoveUser => {
            let user_id = menu::user::get_user_id_input();
            services::user::remove_user(users, user_id);
        }
        MainMenuUserChoice::ListUsers => {
            services::user::list_users(users);
        }
        MainMenuUserChoice::Login => {
            let login_input = menu::user::get_login_input();
            services::user::login(&users, login_input);
        }
        MainMenuUserChoice::Exit => {
            return false;
        }
    }
    true
}

fn run_iteration(users: &mut Vec<User>) -> bool {
    menu::main::print_menu();
    let choice: u8 = menu::main::get_user_input();
    let running: bool;
    if menu::main::validate_user_input(choice) {
        running = match_input(choice, users);
    } else {
        println!("Please type a number between 1 and 4");
        running = run_iteration(users);
    }
    running
}

fn main() {
    let mut users = services::user::get_users_from_file();
    let mut running = true;
    while running {
        running = run_iteration(&mut users);
        if !running {
            break;
        }
    }
}
