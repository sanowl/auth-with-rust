use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use chrono::{NaiveDateTime, Local};
use uuid::Uuid;
use std::convert::From;
use serde::{Serialize, Deserialize};
use diesel::{Queryable, Insertable};

pub struct DbExecutor(pub Pool<ConnectionManager<PgConnection>>);

impl Actor for DbExecutor {
    type Context = SyncContext<Self>;
}

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub email: String,
    pub password: String,
    pub created_at: NaiveDateTime, // only works in here due to diesel limitations
}

impl User {
    pub fn remove_pwd(mut self) -> Self {
        self.password = "".to_string();
        self
    }

    pub fn with_detail(email: String, password: String) -> Self {
        User {
            email,
            password,
            created_at: Local::now().naive_local(),
        }
    }
}

pub struct SlimUser {
    pub email: String,
}

impl From<User> for SlimUser {
    fn from(user: User) -> Self {
        SlimUser {
            email: user.email,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Queryable)]
pub struct Invitation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: NaiveDateTime,
}