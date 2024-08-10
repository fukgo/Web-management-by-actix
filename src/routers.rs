use std::vec;
use diesel::{associations::HasTable, r2d2::ConnectionManager};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use actix_web::{web, HttpResponse, HttpRequest,Scope};
use crate::handlers::user::generate_img_url;
use crate::{errors::EveryError, handlers::user::{delete, get_online_users, login, logout, register,get_user_profile}, schema::product_types_table::type_name};
use crate::handlers::products::*;
use actix_web::web::Data;
use crate::handlers::file_service::upload_file;
use rand::Rng;
pub fn file_routes(cfg: &mut web::ServiceConfig) {
    cfg
    .service(
        web::resource("/upload").route(web::post().to(upload_file)),
    );
    // .service(
        // web::resource("/download").route(web::post().to()),
    // );
}


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
            .route("/profile", web::get().to(get_user_profile))
    );
}
use r2d2::Pool;
// pub async fn product_routes(cfg: &mut web::ServiceConfig, pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>) {
//     let field_names = match get_product_types(pool).await {
//         Ok(result) => result,
//         Err(e) => {
//             eprintln!("Error getting product types: {}", e);
//             return;
//         }
//     };
    
//     for field_name in field_names {
//         let field_name_clone = field_name.clone();
//         println!("field_name: {}", field_name);
//         cfg.service(
//             web::scope(&format!("/{}", field_name_clone))
//                 .route("/", web::get().to(ggggg))
//         );
//     }
// }
pub fn product_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/product")
            .route("/all-types", web::get().to(get_all_product_types))
            .route("/", web::get().to(get_all_product_by_list))
            .route("/delete/{id}", web::get().to(delete_product))
            
    );

}

use crate::schema::product_types_table::dsl::*;
pub async fn get_product_types(pool: web::Data<Pool<ConnectionManager<MysqlConnection>>>) -> Result<Vec<String>, EveryError> {
    let mut conn = pool.get().map_err(|e| EveryError::DatabaseError(e.to_string()))?;
    let type_names: Vec<String> = product_types_table
        .select(type_name)
        .load(&mut conn)?;

    Ok(type_names)
}
