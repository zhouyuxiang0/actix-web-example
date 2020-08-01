use crate::db::DbPool;
use crate::schema::users::dsl::{id as userId, password, username, users as userModel};
use crate::user::models::{NewUser, User};
use diesel::prelude::*;
use juniper::{FieldError, FieldResult, RootNode};
pub struct Context {
    pub dbpool: DbPool,
}

impl juniper::Context for Context {}

pub struct QueryRoot;

#[juniper::object(Context = Context)]
impl QueryRoot {
    #[graphql(description = "获取所有用户")]
    fn fetchUsers(context: &Context) -> FieldResult<Vec<User>> {
        let connect = context.dbpool.get()?;
        let users = userModel.load::<User>(&connect)?;
        Ok(users)
    }

    #[graphql(description = "根据ID获取用户")]
    fn fetchUser(context: &Context, user_id: i32) -> FieldResult<User> {
        let connect = context.dbpool.get()?;
        let user = userModel
            .filter(userId.eq(user_id))
            .first::<User>(&connect)?;
        Ok(user)
    }
}

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
    fn createUser(context: &Context, create_user: NewUser) -> FieldResult<User> {
        let connect = context.dbpool.get()?;
        diesel::insert_into(userModel)
            .values(&create_user)
            .execute(&connect)?;
        let user = userModel
            .filter(username.eq(&create_user.username))
            .filter(password.eq(&create_user.password))
            .first::<User>(&connect)?;
        Ok(user)
    }
}
pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
    Schema::new(QueryRoot, MutationRoot)
}
