use actix_web::{web, App, HttpServer, middleware::Logger, Error, HttpRequest,HttpResponse};
use actix_web::cookie::Key;
use diesel::r2d2::{self, ConnectionManager};
use diesel::mysql::MysqlConnection;
use dotenv::dotenv;
use std::env;
use std::io;
use management::routers::{user_routes,file_routes,product_routes};
use management::middlewares::API_timing_middleware::Timing;
use redis::Connection;
use r2d2_redis::RedisConnectionManager;
use actix_web_actors::ws;
use actix::Actor;
use actix::StreamHandler;
use actix_cors::Cors;
use std::time::Duration;
/// 定义 WebSocket Actor
struct MyWebSocket;

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;
}
/// 实现 StreamHandler trait 来处理 WebSocket 消息
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => ctx.pong(&msg),
            Ok(ws::Message::Text(text)) => ctx.text(text),
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            _ => (),
        }
    }
}
/// 定义 WebSocket 路由
async fn chat_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    let resp = ws::start(MyWebSocket {}, &req, stream);
    println!("{:?}", resp);
    resp
}
#[actix_web::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL 没有在 .env 文件里设置");
    let bind_address = env::var("BIND_ADDRESS").expect("BIND_ADDRESS 没有在 .env 文件里设置");
    let bind_port = env::var("BIND_PORT").expect("BIND_PORT 没有在 .env 文件里设置");

    // 创建一个 ConnectionManager，用于管理数据库连接
    let mysql_manager = ConnectionManager::<MysqlConnection>::new(&database_url);
    let mysql_pool = r2d2::Pool::builder()
        .connection_timeout(std::time::Duration::from_secs(10))//设置连接超时时间为10秒
        .max_size(5)
        .build(mysql_manager)
        .expect("Failed to create mysql_pool.");
    let redis_manager = RedisConnectionManager::new("redis://127.0.0.1:6379").unwrap();
    let redis_pool = r2d2::Pool::builder()
        .max_size(5)
        .build(redis_manager)
        .expect("Failed to create redis_pool.");
    let secret_key = Key::generate();

    HttpServer::new(move || { 
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION, // 允许 Authorization 头,通常用于携带认证令牌。
                        actix_web::http::header::ACCEPT,        // 允许 Accept 头,指示客户端能处理的内容类型。
                        actix_web::http::header::CONTENT_TYPE,  // 允许 Content-Type 头,指示客户端能处理的内容类型。
                    ])
                    .allowed_header(actix_web::http::header::CONTENT_TYPE)
                    .max_age(3600)
            )
            //记录 HTTP 请求和响应的信息
            .wrap(Timing)
            .wrap(Logger::default())
            //web::Data::new用于创建一个新的 Data 实例，它允许你在多个请求之间共享数据。
            .app_data(web::Data::new(mysql_pool.clone()))
            .app_data(web::Data::new(redis_pool.clone()))
            .configure(user_routes)
            .configure(file_routes)
            .configure(product_routes)
            .route("/ws/", web::get().to(chat_route)) // 添加 WebSocket 路由

    })
    .bind(format!("{}:{}", bind_address, bind_port))?
    .run()
    .await
}
