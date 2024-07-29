use actix_web::{web, HttpResponse,HttpRequest};
use actix_web::cookie::Cookie;
use futures_util::{FutureExt, TryFutureExt};
use reqwest::Request;
use crate::models::user_model::{UserCreate, UserDetail,UserLogin};
use crate::errors::EveryError;
use crate::db_operations::product_sql::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use diesel::mysql::MysqlConnection;
use serde::Serialize;

#[derive(Serialize)]
struct Response<T>{
    status:u16,
    data:T,
}
pub async fn get_product_detail(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,product_id: web::Json<i32>) -> Result<HttpResponse, EveryError> {
    let product_id = product_id.into_inner();
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let product_detail = get_product_detail_sql(&mut one_poll, product_id).await?;
    Ok(HttpResponse::Ok().json(Response{status:200,data:product_detail}))
}
pub async fn delete_product(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,product_id: web::Json<i32>) -> Result<HttpResponse, EveryError> {
    let product_id = product_id.into_inner();
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let product_detail = delete_product_sql(&mut one_poll, product_id).await?;
    Ok(HttpResponse::Ok().json(Response{status:200,data:product_detail}))
}

pub async fn get_all_product_types(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>)-> Result<HttpResponse, EveryError> {
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let all_types = get_all_product_types_sql(&mut one_poll).map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string())).await?;
    Ok(HttpResponse::Ok().json(Response{status:200,data:all_types}))
}
