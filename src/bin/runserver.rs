use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::{web, App, HttpServer, middleware::Logger};
use actix_web::cookie::Key;
use diesel::r2d2::{self, ConnectionManager};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use std::io;
use management::routers::user_routes;
use management::middlewares::API_timing_middleware::Timing;
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS 没有在 .env 文件里设置");
    let bind_port = env::var("BIND_PORT").expect("BIND_PORT 没有在 .env 文件里设置");

    // 创建一个 ConnectionManager，用于管理数据库连接
    let manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    let pool = r2d2::Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Failed to create pool.");

    let secret_key = Key::generate();
    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379").await.unwrap();
    HttpServer::new(move || {
        App::new()
            //记录 HTTP 请求和响应的信息
            .wrap(Timing)
            .wrap(Logger::default())
            .wrap(SessionMiddleware::new(redis_store.clone(), secret_key.clone()))
            .app_data(web::Data::new(pool.clone()))
            .configure(user_routes)
    })
    .bind(format!("{}:{}", bind_address, bind_port))?
    .run()
    .await
}
