use actix_web::{middleware, App, HttpServer};
use dotenv::{dotenv, var};
mod db;
mod handlers;
mod schemas;
mod user;
use handlers::register;
use listenfd::ListenFd;
#[macro_use]
extern crate diesel;
extern crate dotenv;
pub mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    std::env::set_var("RUST_LOG", "actix_web=info,info");
    env_logger::init();
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| {
        App::new()
            .data(db::establish_connection().clone())
            .wrap(middleware::Logger::default())
            .configure(register)
    });
    server = if let Some(tcp_listener) = listenfd.take_tcp_listener(0)? {
        server.listen(tcp_listener)?
    } else {
        server.bind(var("PORT").expect("port"))?
    };
    server.run().await
}
