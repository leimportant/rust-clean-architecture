use serde::{Deserialize, Serialize};
use sqlx::FromRow;

use crate::adapters::inbound::http::user;

use sqlx::Type;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Type, Serialize, Deserialize)]
#[sqlx(type_name = "CHAR")]
pub enum YesNo {
    #[sqlx(rename = "Y")]
    Yes,
    #[sqlx(rename = "N")]
    No,
}


impl YesNo {
    pub fn as_bool(self) -> bool {
        matches!(self, YesNo::Yes)
    }
}


#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub name: String,
    pub password: String,
    pub is_active: String,
}

#[derive(Debug, Clone)]
pub struct UserId(pub String);


impl User {
    pub fn new(id: String, email: String, username: String, name: String, password: String, is_active : String) -> Self {
        Self {
            id,
            email,
            username,
            name,
            password,
            is_active,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = "N".to_string();
    }

    pub fn change_password(&mut self, new_password: String) {
        self.password = new_password;
    }
}

