#[macro_use]
extern crate diesel;
#[macro_use]
extern crate serde_derive;

use actix::prelude::*;
use actix_files as fs;
use actix_web::{
    dev,
    http::{
        self,
        header::{ContentDisposition, DispositionType},
    },
    middleware::{
        self,
        errhandlers::{ErrorHandlerResponse, ErrorHandlers},
        Logger,
    },
    web::{self, delete, get, post, put, resource, route, scope},
    App, Error, HttpRequest, HttpResponse, HttpServer, Responder, Result,
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

fn render_500<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        http::header::CONTENT_TYPE,
        http::HeaderValue::from_static("Error"),
    );
    Ok(ErrorHandlerResponse::Response(res))
}

fn index(req: HttpRequest) -> Result<fs::NamedFile, Error> {
    let filename: &str = req.match_info().query("filename");
    let ext: &str = req.match_info().query("ext");
    let file = fs::NamedFile::open(String::from("src/static/") + filename + "." + ext)?;
    Ok(file
        .use_last_modified(true)
        .use_etag(true)
        .set_content_disposition(ContentDisposition {
            disposition: DispositionType::Inline,
            parameters: vec![],
        }))
}

fn index1() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

// systemfd --no-pid -s http::3000 -- cargo watch -x run
fn main() {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();
    let sys = actix::System::new("actix-web-example");
    let addr: DbAddr = init_dba();
    let bind_host = dotenv::var("BIND_ADDRESS").unwrap_or("127.0.0.1:3000".to_string());
    let mut listenfd = ListenFd::from_env();
    let mut server = HttpServer::new(move || {
        App::new()
            .wrap(ErrorHandlers::new().handler(http::StatusCode::INTERNAL_SERVER_ERROR, render_500))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %{User-Agent}i"))
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "0.2"))
            .data(addr.clone())
            .route("/static/{filename}.{ext}", web::get().to(index))
            .service(
                scope("/api").route("/", web::get().to(index1)), // .service(resource("/signup").route(post().to_async(api::auth::signup))),
            )
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
