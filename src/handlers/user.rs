use actix_session::Session;
use actix_web::{web, HttpResponse,HttpRequest};
use diesel::IntoSql;
use crate::models::user_model::{UserCreate, UserDetail,UserQuery};
use crate::errors::EveryError;
use crate::db_operations::user_sql::{post_new_user_sql,login_query_sql};
use diesel::r2d2::ConnectionManager;
use actix_web::cookie::Key;
use diesel::r2d2::Pool;
use diesel::mysql::MysqlConnection;
use actix_web::dev::Extensions;
pub async fn register(
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    new_user: web::Json<UserCreate>,
) -> Result<HttpResponse, EveryError> {
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

pub async fn delete() -> HttpResponse {
    HttpResponse::Ok().body("login")
}
pub async fn update() -> HttpResponse {
    HttpResponse::Ok().body("login")
}
pub async fn logout(session:Session) -> HttpResponse {
    session.purge();
    HttpResponse::Ok().body("loggeed out")
}