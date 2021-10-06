use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::Id;

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct SignUp {
    pub email: String,
    pub name: String,
    pub password: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct SignIn {
    pub login: String,
    pub password: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct SignInResponse {
    pub token: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CreateQueue {
    pub name: String,
    pub description: String,
}

pub mod values {
    pub const fn true_value() -> bool {
        true
    }
}
