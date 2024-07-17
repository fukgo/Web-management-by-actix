use diesel::r2d2::{self, ConnectionManager};
use diesel::mysql::MysqlConnection;
use std::io;
use management::routers::user_routes;
use actix_web::web::Data;
use actix_web::{App, HttpServer, HttpResponse, Error,http, web};
use actix_web::cookie::Key;
use actix_session::{Session, SessionMiddleware,  storage::RedisSessionStore};
use actix_identity::{CookieIdentityPolicy, IdentityService};
#[actix_rt::main]
async fn main()-> io::Result<()> {
    dotenv::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let bind_address = std::env::var("BIND_ADDRESS").expect("BIND_ADDRESS 没有在 .env 文件里设置");
    let bind_port = std::env::var("BIND_PORT").expect("BIND_PORT 没有在 .env 文件里设置");
    //创建了一个ConnectionManager，它是diesel库中的一个组件，用于管理数据库连接
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    //创建了一个数据库连接池，它是一个r2d2库中的组件，用于管理数据库连接// 设置连接池的最大大小为15

    let mult_polls = r2d2::Pool::builder().max_size(5).build(manager).expect("Failed to create pool.");

        // When using `Key::generate()` it is important to initialize outside of the
    // `HttpServer::new` closure. When deployed the secret key should be read from a
    // configuration file or environment variables.
    let secret_key = Key::generate();

    let redis_store = RedisSessionStore::new("redis://127.0.0.1:6379")
        .await
        .unwrap();
    let indentify_service_middleware = IdentityMiddleware::default();
    let session_middleware = SessionMiddleware::new(redis_store.clone(), secret_key.clone());
    let app = move || {
        App::new()
            //将mult_polls（一个数据库连接池）添加到Actix-web应用中，使其可以在处理请求的函数中被访问。
            .app_data(Data::new(mult_polls.clone()))
            //创建了一个新的 session 中间件，并将其添加到应用中
            .wrap(session_middleware)
            //创建了一个新的 identity 中间件，并将其添加到应用中
            .wrap(indentify_service_middleware)
            .configure(user_routes)
    };

    HttpServer::new(app).bind(format!("{}:{}", bind_address, bind_port))?.run().await
    

}