use crate::tools::parse_input_to_u8;

pub(crate) fn print_menu() {
    println!("1. Add user");
    println!("2. Remove user");
    println!("3. List users");
    println!("4. Login");
    println!("5. Exit");
}

pub(crate) fn validate_user_input(input: u8) -> bool {
    if input < 1 || input > 5 {
        false
    } else {
        true
    }
}

pub(crate) fn get_user_input() -> u8 {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    parse_input_to_u8(input)
}