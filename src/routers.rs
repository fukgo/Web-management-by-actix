use actix_web::{web, HttpResponse, HttpRequest};
use crate::handlers::user::{register,login,logout};
pub fn user_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/user")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/logout", web::post().to(logout))
            //.route("/info", web::get().to(get_user_info))
            //.route("/update", web::post().to(update_user_info))
            //.route("/delete", web::post().to(delete_user))
    );
}