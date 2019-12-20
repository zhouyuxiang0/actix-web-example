use actix::Message;
use chrono::{NaiveDateTime};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Identifiable, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
  pub id: String,
  pub uname: String,
  pub password: String,
  pub create_time: NaiveDateTime,
  pub email: String,
  pub nickname: String,
  pub permission: i16,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegUser {
  pub uname: String,
  pub password: String,
}

impl Message for RegUser {
  type Result = Result<Msg, ServiceError>;
}