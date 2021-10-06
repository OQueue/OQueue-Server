use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub type Id = Uuid;

// ---------
// UserId
// ---------

#[derive(Clone, Hash, Debug)]
pub struct UserId(pub Id);

// ---------
// QueueId
// ---------

#[derive(Clone, Hash, Debug)]
pub struct QueueId(pub Id);

// --------
// User
// --------

#[derive(Clone, Hash, Debug)]
pub struct User {
    id: UserId,
    name: String,
    email: String,
    pwhash: String,
}

// -------
// Other Structures
// -------

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct UserInfo {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct MemberInfo {
    pub id: Uuid,
    pub order: i32,
    pub has_priority: bool,
    pub is_held: bool,
    pub joined_at: NaiveDateTime,
}

#[derive(Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct QueueInfo {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub organizer_id: Uuid,
    pub created_at: NaiveDateTime,
    pub exists_before: NaiveDateTime,
}
