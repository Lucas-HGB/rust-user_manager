use std::fs::File;
use std::path::Path;

use crate::structures::user::User;

pub(crate) fn list_users(users: &Vec<User>) {
    let mut index = 0;
    if users.is_empty() {
        println!("No users found");
        return;
    }
    for user in users {
        println!("--------------------------------");
        println!("ID: {}, Name: '{}', Age: {}", user.id, user.name, user.age);
        if index == users.len() - 1 {
            println!("--------------------------------");
        }
        index += 1;
    }
}

pub(crate) fn get_users_from_file() -> Vec<User> {
    let file = File::open("users.json");
    if file.is_err() {
        return Vec::new();
    }
    let users: Vec<User> = serde_json::from_reader(file.unwrap()).unwrap();
    users
}

pub(crate) fn login(users: &Vec<User>, login_input: (String, String)) -> bool {
    let user = users.iter().find(|user| user.name == login_input.0);
    if user.is_none() {
        println!("User not found");
        return false;
    }
    if user.unwrap().is_password_correct(login_input.1) {
        println!("Login successful");
        return true;
    }
    println!("Invalid password");
    false
}

pub(crate) fn add_user(users: &mut Vec<User>, user_creation_input: (String, String, u32)) {
    let user = User::new(
        users.len() as u32 + 1,
        user_creation_input.0,
        user_creation_input.1,
        user_creation_input.2,
    );
    users.push(user);
    save_users_to_file(users.clone());
}

pub(crate) fn remove_user(users: &mut Vec<User>, user_id: u32) {
    users.retain(|user| user.id != user_id);
    save_users_to_file(users.clone());
}

fn save_users_to_file(users: Vec<User>) {
    let file = File::create("users.json").unwrap();
    let err = serde_json::to_writer_pretty(file, &users).unwrap();
}