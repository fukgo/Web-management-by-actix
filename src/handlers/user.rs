use actix_session::Session;
use actix_web::{web, HttpResponse,HttpRequest};
use diesel::IntoSql;
use crate::models::user_model::{UserCreate, UserDetail,UserQuery};
use crate::errors::EveryError;
use crate::db_operations::user_sql::{post_new_user_sql,login_query_sql,delete_user_sql};
use diesel::r2d2::ConnectionManager;
use actix_web::cookie::Key;
use diesel::r2d2::Pool;
use diesel::mysql::MysqlConnection;
use actix_web::dev::Extensions;
use serde::Serialize;

#[derive(Serialize)]
struct Response<T>{
    status:u16,
    data:T
}


pub async fn register(
    session: Session,
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    new_user: web::Json<UserCreate>,
) -> Result<HttpResponse, EveryError> {
    if let Ok(Some(_)) = session.get::<String>("session_uuid") {
        return Ok(HttpResponse::Ok().json("已经登录"));
    }
    let user_create = new_user.into_inner();
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let new_user_vec = post_new_user_sql(&mut one_poll, user_create).await?;
    Ok(HttpResponse::Ok().json(new_user_vec))
}
pub async fn login(
    session: Session,
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    new_user: web::Json<UserQuery>,
    request: HttpRequest
) -> Result<HttpResponse, EveryError> {
    let user_query = new_user.into_inner();
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let match_user = login_query_sql(&mut one_poll, user_query).await?;
    let value = match_user.uuid.to_string();
    session.insert("session_uuid", value);
    Ok(HttpResponse::Ok().json(match_user))

}

pub async fn delete(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    user_uuid:String,) -> HttpResponse {
        match delete_user_sql(&mut pool.get().expect("couldn't get db connection from pool"),user_uuid).await{
            Ok(res) => HttpResponse::Ok().json(Response{status:200,data:res}),
            Err(_) => HttpResponse::Ok().json(Response{status:500,data:"删除失败"})
        }
}
pub async fn update() -> HttpResponse {
    HttpResponse::Ok().body("login")
}
pub async fn logout(session:Session) -> HttpResponse {
    session.purge();
    HttpResponse::Ok().body("loggeed out")
}