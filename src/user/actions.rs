use super::dto::LoginDto;
use super::models;
use crate::schema::users::dsl::{username, users};
use diesel::prelude::*;
use diesel::result::Error;

pub fn insert_new_user(
    login_dto: LoginDto,
    connect: &MysqlConnection,
) -> Result<Option<models::User>, Error> {
    let new_user = models::NewUser {
        username: login_dto.username,
        password: login_dto.password,
        // create_time: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11),
        // update_time: NaiveDate::from_ymd(2016, 7, 8).and_hms(9, 10, 11)
    };
    diesel::insert_into(users)
        .values(&new_user)
        .execute(connect)?;
    let user = users
        .filter(username.eq(new_user.username))
        .first::<models::User>(connect)
        .optional()?;
    Ok(user)
}

// pub fn find_user_by_pwd() {}
