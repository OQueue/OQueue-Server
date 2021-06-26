use std::borrow::Borrow;
use std::ops::Deref;

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::result::QueryResult;
use diesel::PgConnection;
use r2d2::{Pool, PooledConnection};
use uuid::Uuid;

use crate::db::actions::QueueEntryToAdd;
use crate::db::models::{QueueDao, QueueEntryDao, UserDao};

pub mod models;
mod schema;

pub mod actions;

pub type DbConnection = PgConnection;
pub type DbPool = Pool<ConnectionManager<DbConnection>>;

// -----
// Error
// -----
#[derive(Debug)]
pub enum Error {
    R2D2(r2d2::Error),
    Query(diesel::result::Error),
}

impl From<r2d2::Error> for Error {
    fn from(e: r2d2::Error) -> Self {
        Self::R2D2(e)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Self::Query(e)
    }
}

type Result<T> = std::result::Result<T, Error>;

// ---------
// DbService
// ---------

#[derive(Clone)]
pub struct DbService {
    pool: DbPool,
}

impl DbService {
    pub fn new(pool: DbPool) -> Self {
        DbService { pool }
    }

    pub fn conn(&self) -> Result<PooledConnection<ConnectionManager<DbConnection>>> {
        let c = self.pool.get()?;
        Ok(c)
    }

    // ----
    // User
    // ----

    pub fn add_user(&self, user_data: &UserDao) -> Result<()> {
        let conn = &*self.conn()?;
        actions::add_user(conn, user_data)?;
        Ok(())
    }

    pub fn user_by_email(&self, email_str: &str) -> Result<Option<UserDao>> {
        let conn = &*self.conn()?;
        Ok(actions::user_by_email(conn, email_str)?)
    }

    pub fn user_by_id(&self, user_id: &Uuid) -> Result<Option<UserDao>> {
        let conn = &*self.conn()?;
        Ok(actions::user_by_id(conn, user_id)?)
    }

    pub fn has_user_with_email(&self, email_str: &str) -> Result<bool> {
        let conn = &*self.conn()?;
        Ok(actions::has_user_with_email(conn, email_str)?)
    }

    // ------
    // Queue
    // ------

    pub fn add_queue(&self, queue_data: &QueueDao) -> Result<()> {
        let conn = &*self.conn()?;
        actions::add_queue(conn, queue_data)?;
        Ok(())
    }

    pub fn delete_queue(&self, queue_id: &Uuid) -> Result<()> {
        let conn = &*self.conn()?;
        actions::delete_queue(conn, queue_id)?;
        Ok(())
    }

    pub fn queue_by_id(&self, queue_id: &Uuid) -> Result<Option<QueueDao>> {
        let conn = &*self.conn()?;
        Ok(actions::queue_by_id(conn, queue_id)?)
    }

    pub fn queues_with_member(&self, user_id: &Uuid) -> Result<Vec<QueueDao>> {
        let conn = &*self.conn()?;
        Ok(actions::queues_with_member(conn, user_id)?)
    }

    // ------------
    // QueueEntry
    // ------------

    pub fn add_entry(&self, entry: &QueueEntryToAdd) -> Result<()> {
        let conn = &*self.conn()?;
        actions::add_entry(conn, entry)?;
        Ok(())
    }

    pub fn delete_entry(&self, queue_id: &Uuid, user_id: &Uuid) -> Result<()> {
        let conn = &*self.conn()?;
        actions::delete_entry(conn, queue_id, user_id)?;
        Ok(())
    }

    pub fn entries_ordered(&self, queue_id: &Uuid) -> Result<Vec<QueueEntryDao>> {
        let conn = &*self.conn()?;
        Ok(actions::entries_ordered(conn, queue_id)?)
    }
}
