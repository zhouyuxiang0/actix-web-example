use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub username: String,
    pub password: String,
    // pub create_time: NaiveDateTime,
    // pub update_time: NaiveDateTime
}

// impl Crud<NewUser> for User {}
