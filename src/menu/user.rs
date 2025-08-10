use crate::tools::parse_input_to_u32;

pub(crate) fn get_user_id_input() -> u32 {
    println!("Enter the id of the user: ");
    let mut id = String::new();
    std::io::stdin().read_line(&mut id).unwrap();
    parse_input_to_u32(id)
}

pub(crate) fn get_user_creation_input() -> (String, String, u32) {
    println!("Enter the name of the user: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Enter the password of the user: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    println!("Enter the age of the user: ");
    let mut age = String::new();
    std::io::stdin().read_line(&mut age).unwrap();
    (
        name.trim().to_string(),
        password.trim().to_string(),
        parse_input_to_u32(age),
    )
}

pub(crate) fn get_login_input() -> (String, String) {
    println!("Enter name of the user: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name).unwrap();
    println!("Enter password of the user: ");
    let mut password = String::new();
    std::io::stdin().read_line(&mut password).unwrap();
    (name.trim().to_string(), password.trim().to_string())
}