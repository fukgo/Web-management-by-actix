use actix_web::{web, HttpResponse};
use crate::models::user_model::{UserCreate, UserDetail,UserQuery};
use crate::errors::EveryError;
use crate::db_operations::user_sql::{post_new_user_sql,login_query_sql};
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::mysql::MysqlConnection;

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
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    new_user: web::Json<UserQuery>,
) -> Result<HttpResponse, EveryError> {
    let user_query = new_user.into_inner();
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let match_user = login_query_sql(&mut one_poll, user_query).await?;
    Ok(HttpResponse::Ok().json(match_user))
}

pub async fn delete() -> HttpResponse {
    HttpResponse::Ok().body("login")
}
pub async fn update() -> HttpResponse {
    HttpResponse::Ok().body("login")
}
pub async fn logout() -> HttpResponse {
    HttpResponse::Ok().body("login")
}