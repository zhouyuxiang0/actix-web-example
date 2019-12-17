use actix::prelude::*;
use actix_rt::System;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use listenfd::ListenFd;

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
    // let sys = System::new("actix_web_example");
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(|| App::new().route("/", web::get().to(index)));
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l).unwrap()
    } else {
        server.bind("127.0.0.1:3000").unwrap()
    };
    server.run().unwrap();
}
