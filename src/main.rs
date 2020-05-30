use actix_web::{web, App, HttpResponse, HttpServer};
use dotenv::{dotenv, var};
mod db;
mod user;
use listenfd::ListenFd;
#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(db::establish_connection().clone())
            .service(user::api::login)
    });
    server = if let Some(tcp_listener) = listenfd.take_tcp_listener(0)? {
        server.listen(tcp_listener)?
    } else {
        server.bind(var("PORT").expect("port"))?
    };
    println!(
        "Service listening on http://localhost:{}",
        var("PORT").expect("port")
    );
    server.run().await
}
