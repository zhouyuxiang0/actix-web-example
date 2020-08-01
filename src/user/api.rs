use super::actions;
use super::dto::LoginDto;
use crate::db::DbPool;
use actix_web::{post, web, HttpResponse, Responder};

#[post("/user")]
pub async fn login(pool: web::Data<DbPool>, login_dto: web::Json<LoginDto>) -> impl Responder {
    let conn = pool.get().expect("无法从连接池中获取连接");
    let login_dto = login_dto.into_inner();
    match actions::insert_new_user(login_dto, &conn) {
        Ok(user) => HttpResponse::Ok().body(format!("{:?}", user)),
        _ => HttpResponse::Ok().body("error"),
    }
}
