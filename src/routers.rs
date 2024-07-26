use std::vec;
use diesel::{associations::HasTable, r2d2::ConnectionManager};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, HttpRequest,Scope};
use crate::{errors::EveryError, handlers::user::{delete, get_online_users, login, logout, register}, schema::product_types_table::type_name};
use crate::handlers::products::*;
use actix_web::web::Data;
pub fn user_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
web::scope("/user")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::get().to(logout))
            .route("/all_online_users", web::get().to(get_online_users))
            //.route("/info", web::get().to(get_user_info))
            //.route("/update", web::post().to(update_user_info))
            .route("/delete", web::post().to(delete))
    );
}
use r2d2::Pool;
pub async fn product_routes(cfg: &mut web::ServiceConfig, pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>) {
    let field_names = get_product_types(pool).await
        .map_err(|e| actix_web::error::ErrorInternalServerError(e.to_string()))
        .unwrap();
    
    for field_name in field_names {
        let field_name_clone = field_name.clone();
        cfg.service(
            web::scope(&format!("/{}", field_name_clone))
                .route("/", web::get().to(get_product_detail))
                .route("/delete", web::get().to(delete_product))
        );

    }
}


use crate::schema::product_types_table::dsl::*;
pub async fn get_product_types(pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>) -> Result<Vec<String>, EveryError> {
    let mut conn = pool.get().map_err(|e| EveryError::DatabaseError(e.to_string()))?;
    let type_names: Vec<String> = product_types_table
        .select(type_name)
        .load(&mut conn)?;

    Ok(type_names)
}