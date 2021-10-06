use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::schema::*;

#[derive(Clone, Debug, Queryable, Insertable)]
#[table_name = "users"]
pub struct UserDao {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub pwhash: String,
}

#[derive(Clone, Debug, Queryable, Insertable)]
#[table_name = "queues"]
pub struct QueueDao {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub organizer_id: Uuid,
    pub created_at: NaiveDateTime,
    pub exists_before: NaiveDateTime,
}

#[derive(Clone, Debug, Queryable, Insertable)]
#[table_name = "queue_entries"]
pub struct QueueEntryDao {
    pub queue_id: Uuid,
    pub user_id: Uuid,
    pub order: i32,
    pub has_priority: bool,
    pub is_held: bool,
    pub joined_at: NaiveDateTime,
}
