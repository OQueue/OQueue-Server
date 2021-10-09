use std::collections::HashSet;
use chrono::NaiveDateTime;
use diesel::prelude::*;
use uuid::Uuid;

use crate::db::models::{QueueDao, QueueEntryDao, UserDao};
use crate::db::DbConnection;

type Result<T> = QueryResult<T>;

// ----
// User
// ----

pub fn add_user(conn: &DbConnection, user_data: &UserDao) -> QueryResult<usize> {
    use crate::db::schema::users::dsl::*;
    diesel::insert_into(users).values(user_data).execute(conn)
}

pub fn user_by_email(conn: &DbConnection, email_str: &str) -> QueryResult<Option<UserDao>> {
    use crate::db::schema::users::dsl::*;
    let user = users
        .filter(email.eq(email_str))
        .first::<UserDao>(conn)
        .optional();
    user
}

pub fn has_user_with_email(conn: &DbConnection, email_str: &str) -> QueryResult<bool> {
    user_by_email(conn, email_str).map(|x| x.is_some())
}

pub fn user_by_id(conn: &DbConnection, user_id: &Uuid) -> QueryResult<Option<UserDao>> {
    use crate::db::schema::users::dsl::*;
    let user = users
        .filter(id.eq(user_id))
        .first::<UserDao>(conn)
        .optional();
    user
}

// ------
// Queue
// ------

pub fn add_queue(conn: &DbConnection, queue_data: &QueueDao) -> QueryResult<usize> {
    use crate::db::schema::queues::dsl::*;
    diesel::insert_into(queues).values(queue_data).execute(conn)
}

pub fn delete_queue(conn: &DbConnection, queue_id: &Uuid) -> QueryResult<usize> {
    use crate::db::schema::queues::dsl as q;
    diesel::delete(q::queues.filter(q::id.eq(&queue_id))).execute(conn)
}

pub fn queue_by_id(conn: &DbConnection, queue_id: &Uuid) -> QueryResult<Option<QueueDao>> {
    use crate::db::schema::queues::dsl::*;
    let queue = queues
        .filter(id.eq(queue_id))
        .first::<QueueDao>(conn)
        .optional();
    queue
}

pub fn queues_with_member(conn: &DbConnection, user_id: &Uuid) -> QueryResult<Vec<QueueDao>> {
    use crate::db::schema::*;
    let queues = queues::table
        .inner_join(queue_entries::table)
        .filter(queue_entries::user_id.eq(user_id))
        .load::<(QueueDao, QueueEntryDao)>(conn);
    queues.map(|v| v.into_iter().map(|(q, _)| q).collect())
}

pub fn available_queues(conn: &DbConnection, user_id: &Uuid) -> QueryResult<Vec<QueueDao>> {
    use crate::db::schema::*;

    let queues_with_organizer: Vec<QueueDao> = queues::table
        .filter(queues::organizer_id.eq(user_id))
        .load::<QueueDao>(conn)?;

    let queues_with_members: Vec<(QueueDao, QueueEntryDao)> = queues::table
        .inner_join(queue_entries::table)
        .filter(queue_entries::user_id.eq(user_id))
        .load::<(QueueDao, QueueEntryDao)>(conn)?;

    let mut queues = Vec::new();
    let mut queues_uuids = HashSet::new();

    let mut add_or_ignore = |q: QueueDao| {
        let has_copy = queues_uuids.contains(&q.id);
        if !has_copy {
            queues_uuids.insert(q.id);
            queues.push(q);
        }
    };

    for q in queues_with_organizer { add_or_ignore(q); }
    for (q, _) in queues_with_members { add_or_ignore(q); }

    Ok(queues)
}

// ------------
// QueueMembers
// ------------

#[derive(Clone, Debug)]
pub struct QueueEntryToAdd {
    pub queue_id: Uuid,
    pub user_id: Uuid,
    pub has_priority: bool,
    pub joined_at: NaiveDateTime,
}

pub fn add_entry_raw(conn: &DbConnection, data: &QueueEntryDao) -> QueryResult<usize> {
    use crate::db::schema::queue_entries::dsl::*;
    diesel::insert_into(queue_entries)
        .values(data)
        .execute(conn)
}

pub fn add_entry(conn: &DbConnection, data: &QueueEntryToAdd) -> QueryResult<usize> {
    use crate::db::schema::queue_entries::dsl as qe;

    let new_order: i32 = qe::queue_entries
        .select(diesel::dsl::max(qe::order))
        .filter(qe::queue_id.eq(&data.queue_id))
        .first::<Option<i32>>(conn)
        .optional()?
        .flatten()
        .map(|x| x + 1)
        .unwrap_or(0);

    let entry = QueueEntryDao {
        queue_id: data.queue_id,
        user_id: data.user_id,
        order: new_order,
        has_priority: data.has_priority,
        is_held: false,
        joined_at: data.joined_at,
    };

    diesel::insert_into(qe::queue_entries)
        .values(entry)
        .execute(conn)
}

pub fn delete_entry(conn: &DbConnection, queue_id: &Uuid, member_id: &Uuid) -> QueryResult<()> {
    use crate::db::schema::queue_entries as qe;

    let to_del = qe::table.filter(qe::queue_id.eq(&queue_id).and(qe::user_id.eq(&member_id)));

    diesel::delete(to_del).execute(conn)?;

    Ok(())
}

pub fn entries_ordered(conn: &DbConnection, q_id: &Uuid) -> QueryResult<Vec<QueueEntryDao>> {
    use crate::db::schema::queue_entries::dsl as qe;

    let entries = qe::queue_entries
        .filter(qe::queue_id.eq(q_id))
        .order_by((qe::is_held.desc(), qe::order))
        .load::<QueueEntryDao>(conn);
    entries
}

// ----------
//
// ----------
