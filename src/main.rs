#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix::prelude::*;
use actix_web::{
    web::{self, delete, get, post, put, resource, route, scope},
    App, HttpResponse, HttpServer, Responder,
};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use listenfd::ListenFd;

mod api;

pub struct Dba(pub Pool<ConnectionManager<MysqlConnection>>);

impl Actor for Dba {
    type Context = SyncContext<Self>;
}

pub type DbAddr = Addr<Dba>;

pub fn init_dba() -> DbAddr {
    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL不存在");
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let cpu_num = num_cpus::get();
    let pool_num = std::cmp::max(10, cpu_num * 2 + 1) as u32;
    let conn = Pool::builder()
        .max_size(pool_num)
        .build(manager)
        .expect("创建线程池失败");
    SyncArbiter::start(cpu_num * 2 + 1, move || Dba(conn.clone()))
}

fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// systemfd --no-pid -s http::3000 -- cargo watch -x run
fn main() {
    let sys = actix::System::new("actix-web-example");
    let addr: DbAddr = init_dba();
    let bind_host = dotenv::var("BIND_ADDRESS").unwrap_or("127.0.0.1:3000".to_string());
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .data(addr.clone())
            .service(
                scope("/api")
                    // .service(resource("/signup").route(post().to_async(api::auth::signup))),
            )
            .route("/", web::get().to(index))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind(&bind_host).unwrap()
    };
    server.start();
    println!("Starting http server: {}", bind_host);
    let _ = sys.run();
}
