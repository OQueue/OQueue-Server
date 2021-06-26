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
    pub email: String,
    pub password: String,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetUserInfo {
    pub id: Uuid,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetMemberInfo {
    pub queue_id: Uuid,
    pub member_id: Uuid,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct CreateQueue {
    pub name: String,
    pub description: String,

    /// Make current user organizer if true
    #[serde(default = "values::true_value")]
    pub add_organizer: bool,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct DeleteQueue {
    pub id: Uuid,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetQueue {
    pub id: Uuid,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetMembers {
    pub id: Uuid,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct JoinToQueue {
    pub id: Uuid,
}

#[derive(Clone, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct LeaveFromQueue {
    pub id: Uuid,
}

pub mod values {
    pub const fn true_value() -> bool {
        true
    }
}
