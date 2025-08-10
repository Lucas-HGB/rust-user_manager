pub(crate) enum MainMenuUserChoice {
    AddUser = 1,
    RemoveUser = 2,
    ListUsers = 3,
    Login = 4,
    Exit = 5,
}

impl MainMenuUserChoice {
    pub(crate) fn from_value(value: u8) -> MainMenuUserChoice {
        match value {
            1 => MainMenuUserChoice::AddUser,
            2 => MainMenuUserChoice::RemoveUser,
            3 => MainMenuUserChoice::ListUsers,
            4 => MainMenuUserChoice::Login,
            5 => MainMenuUserChoice::Exit,
            _ => panic!("Invalid value for MainMenuUserChoice"),
        }
    }
}