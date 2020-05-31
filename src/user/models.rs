use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::schema::users;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
#[derive(juniper::GraphQLObject)]
#[graphql(description = "user")]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    pub create_time: NaiveDateTime,
    pub update_time: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
#[derive(juniper::GraphQLInputObject)]
#[graphql(description = "创建用户")]
pub struct NewUser {
    pub username: String,
    pub password: String,
}

// impl Crud<NewUser> for User {}
