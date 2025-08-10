use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub(crate) struct User {
    pub(crate) id: u32,
    pub(crate) name: String,
    password: String,
    pub(crate) age: u32,
}

impl User {
    pub(crate) fn new(id: u32, name: String, password: String, age: u32) -> User {
        User {
            id,
            name,
            password,
            age,
        }
    }
    pub(crate) fn is_password_correct(&self, password: String) -> bool {
        self.password == password
    }
}
