use actix_web::{web, HttpResponse,HttpRequest,Responder};
use actix_web::cookie::Cookie;
use rand::Rng;
use reqwest::Request;
use crate::models::user_model::{UserCreate, UserDetail,UserLogin};
use crate::errors::EveryError;
use crate::db_operations::user_sql::{post_new_user_sql,login_query_sql,delete_user_sql,valide_email,get_user_profile_sql};
use diesel::r2d2::ConnectionManager;
use diesel::mysql::MysqlConnection;
use serde::Serialize;
use lazy_static::lazy_static;
use std::collections::HashSet;
use std::sync::Mutex;
use std::vec;
use regex::Regex;
use std::collections::HashMap;
use crate::utils::generate_code;
use actix_web::http::header;
use rand::distributions;
#[derive(Serialize)]
struct Response<T>{
    status:u16,
    data:T,
}
#[derive(Debug,Serialize)]
pub struct UserSession{
    pub count:u32,
    pub uuid_hash_set:HashSet<String>,
}
impl UserSession{
    pub fn new()->Self{
        UserSession{
            count:0,
            uuid_hash_set:HashSet::new(),
        }
    }
    pub fn insert(&mut self,uuid:String){
        if self.uuid_hash_set.insert(uuid) {
            self.count += 1;
        }
    }
    pub fn remove(&mut self,uuid:&String){
        if self.uuid_hash_set.remove(uuid) && self.count > 0 {
            self.count -= 1;
        }
    }

}
//使用了 lazy_static! 宏来创建一个全局的 Mutex<UserSession>
lazy_static! {
    static ref ONLINE_USERS: Mutex<UserSession> = Mutex::new(UserSession::new());
}


//impl Responder代表任何可以转换为 HttpResponse 的类型
pub async fn generate_img_url(img:Vec::<u8>)->impl Responder{
    HttpResponse::Ok()
        .header(header::CONTENT_TYPE, "image/jpeg")
        .body(img)
}


pub async fn register(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,new_user: web::Json<UserCreate>,) -> Result<HttpResponse, EveryError> {
    let user_create = new_user.into_inner();
    let mut one_poll = pool.get().expect("couldn't get db connection from pool");
    let new_user_vec = post_new_user_sql(&mut one_poll, user_create).await?;
    Ok(HttpResponse::Ok().json(Response{status:200,data:new_user_vec}))
}
pub async fn login(
    pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,
    query_user: web::Json<UserLogin>,
) -> Result<HttpResponse, EveryError> {
    let login_user = query_user.into_inner();
    //判断要登陆的uuid是否在在线用户集合中

    let mut conn = pool.get().map_err(|e|EveryError::DatabaseError(e.to_string()))?;
    let match_user = login_query_sql(&mut conn, login_user).await?;
    let user_uuid = match_user.uuid.clone();
    {
        let mut online_users = ONLINE_USERS.lock().unwrap();
        if !online_users.uuid_hash_set.contains(&user_uuid) {
            online_users.insert(user_uuid.clone());
        }
    }
    
    // 创建一个新的会话，将用户的 UUID 作为会话数据插入到会话中
    let cookie = Cookie::build("uuid", user_uuid.clone())
        .path("/")  // 适用于整个网站
        .secure(false)// 如果使用 HTTPS，请设置为 true
        .http_only(true)// 确保 Cookie 不能被 JavaScript 访问
        .finish();


    // 返回一个包含用户信息的 JSON 响应，同时设置一个名为 uuid 的 Cookie
    Ok(HttpResponse::Ok()
        .cookie(cookie)
        .json(Response{status:200,data:match_user})
    )


}

pub async fn delete(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>, user_uuid_json: web::Json<String>) -> HttpResponse {
    let user_uuid_string = user_uuid_json.into_inner(); // 将 Json<String> 解引用为 String
    match delete_user_sql(&mut pool.get().expect("couldn't get db connection from pool"), user_uuid_string).await {
        Ok(res) => HttpResponse::Ok().json(Response{status:200,data:res}),
        Err(_) => HttpResponse::Ok().json(Response{status:500,data:"删除失败"})
    }
}
pub async fn refind_password(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>, input_email: web::Json<String>) -> HttpResponse {
    let email = input_email.into_inner();
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    if email_regex.is_match(&email) {
        match valide_email(&mut pool.get().expect("couldn't get db connection from pool"), email).await {
            Ok(res) => {
                if res {
                    return HttpResponse::Ok().json(Response{status:200,data:"邮箱验证成功"})
                } else {
                    return HttpResponse::Ok().json(Response{status:400,data:"邮箱不存在"})
                }
            }
            Err(_) => {
                return HttpResponse::Ok().json(Response{status:500,data:"邮箱验证失败"})
            }
        }
    } else {
        return HttpResponse::Ok().json(Response{status:400,data:"邮箱格式错误"})
    }
}
pub async fn logout(req:HttpRequest) -> HttpResponse {
    match req.cookie("uuid") {
        Some(cookie) => {
            let user_uuid_string = cookie.value().to_string();
            // 用户退出登录，将他们的 UUID 从在线用户集合中删除
            {
                let mut online_users = ONLINE_USERS.lock().unwrap();
                online_users.remove(&user_uuid_string);
            }
            HttpResponse::Ok().body("login out")

        }
        none => {
            HttpResponse::Ok().body("没有登录")
        }
    }

}
pub async fn get_online_users() -> HttpResponse {
    let online_users = ONLINE_USERS.lock().unwrap();
    // let user_sessions = UserSession{
    //     count:online_users.count,
    //     uuid_hash_set:online_users.uuid_hash_set.clone(),
    // };
    //或者
    let user_sessions: Vec<_> = online_users.uuid_hash_set.iter().cloned().collect();
    HttpResponse::Ok().json(Response{status:200,data:user_sessions})
}
pub async fn update_user(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,new_user: web::Json<UserCreate>) {
    
}

pub async fn get_user_profile(pool: web::Data<r2d2::Pool<ConnectionManager<MysqlConnection>>>,user_uuid: web::Query<String>)->Result<HttpResponse,EveryError>{
    let user_uuid = user_uuid.into_inner();
    let mut conn = pool.get().map_err(|e|EveryError::DatabaseError(e.to_string()))?;
    let user_profile = get_user_profile_sql(&mut conn,user_uuid).await?;
    Ok(HttpResponse::Ok().json(Response{status:200,data:user_profile}))
}