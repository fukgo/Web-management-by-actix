use crate::db_operations::product_sql::*;
use crate::errors::EveryError;
use crate::models::product_model::{ProductCreate, ProductTypesCreate};
use crate::models::user_model::{UserCreate, UserDetail, UserLogin};
use actix_web::cookie::Cookie;
use actix_web::{web, HttpRequest, HttpResponse};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use futures_util::{FutureExt, TryFutureExt};
use reqwest::Request;
use serde::Serialize;

#[derive(Serialize)]
struct Response<T> {
    status: u16,
    data: T,
}

pub async fn post_product_type(
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    new_product_type: web::Json<ProductTypesCreate>,
) -> Result<HttpResponse, EveryError> {
    let mut one_pool = pool.get().expect("couldn't get db connection from pool");
    let product_type_detail =
        post_product_type_sql(&mut one_pool, new_product_type.into_inner()).await?;
    Ok(HttpResponse::Ok().json(Response {
        status: 200,
        data: product_type_detail,
    }))
}
pub async fn post_product(
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    new_product: web::Json<ProductCreate>,
) -> Result<HttpResponse, EveryError> {
    let mut one_pool = pool.get().expect("couldn't get db connection from pool");
    let product_detail = post_new_product_sql(&mut one_pool, new_product.into_inner()).await?;
    Ok(HttpResponse::Ok().json(Response {
        status: 200,
        data: product_detail,
    }))
}
pub async fn get_product_detail(
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    product_id: web::Json<i32>,
) -> Result<HttpResponse, EveryError> {
    let product_id = product_id.into_inner();
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let product_detail = get_product_detail_sql(&mut one_poll, product_id).await?;
    Ok(HttpResponse::Ok().json(Response {
        status: 200,
        data: product_detail,
    }))
}
pub async fn delete_product(
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    product_id: web::Json<i32>,
) -> Result<HttpResponse, EveryError> {
    let product_id = product_id.into_inner();
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let product_detail = delete_product_sql(&mut one_poll, product_id).await?;
    Ok(HttpResponse::Ok().json(Response {
        status: 200,
        data: product_detail,
    }))
}

pub async fn get_all_product_types(
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
) -> Result<HttpResponse, EveryError> {
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let all_types = get_all_product_types_sql(&mut one_poll)
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
        .await?;
    Ok(HttpResponse::Ok().json(Response {
        status: 200,
        data: all_types,
    }))
}
#[derive(Deserialize)]
struct PageListQueryParams {
    //页码
    page: Option<u32>,
    //每页显示数量
    page_size: Option<u32>,
}

//产品列表，每页显示20个产品
pub async fn get_all_product_by_list(
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    query: web::Query<PageListQueryParams>,
) -> Result<HttpResponse, EveryError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(20);
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let product_list = get_all_product_by_list_sql(&mut one_poll, page, page_size).await;
    match product_list {
        Ok(product_list) => {
            let response = Response {
                status: 200,
                data: product_list,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Err(e) => {
            let response = Response {
                status: 500,
                data: e.to_string(),
            };
            Ok(HttpResponse::Ok().json(response))
        }
    }
}
