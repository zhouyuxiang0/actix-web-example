use diesel::prelude::*;
use diesel::result::Error;
use crate::models;

pub fn insert_new_user(username: &str, password: &str, connect: &MysqlConnection) -> Result<User, Error> {
    use crate::schema::users::dsl::*;
    let new_user = models::NewUser {
        username: username.to_owned(),
        password: password.to_owned()
    };
    diesel::insert_into(users).values(&new_user).execute(connect)?;
    Ok(new_user)
}

pub fn find_user_by_pwd() {}
