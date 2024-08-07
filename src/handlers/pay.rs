use actix_web::{web, HttpResponse,HttpRequest};
use actix_web::cookie::Cookie;
use futures_util::{FutureExt, TryFutureExt};
use reqwest::Request;
use crate::models::product_model::{ProductTypesCreate,ProductCreate};
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