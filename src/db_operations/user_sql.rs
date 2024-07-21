use diesel::mysql::Mysql;
use uuid::Uuid;
use crate::errors::EveryError;
use crate::models::user_model::{UserDetail,UserCreate,UserQuery,UserCreateWithUuid};
use diesel::MysqlConnection;
use actix_session::Session;
//post新用户，返回
pub async fn post_new_user_sql(pool: &mut MysqlConnection, new_user: UserCreate) -> Result<UserDetail, EveryError> {
    use crate::schema::users_table;
    use diesel::prelude::*;
    let query_username = new_user.username.clone();
    let query_email = new_user.email.clone();
    //查询用户名和邮箱是否已经存在
    let user: UserDetail = users_table::table
        .filter(users_table::username.eq(&query_username).or(users_table::email.eq(&query_email)))
        .first(pool)?;
    //如果用户名和邮箱已经存在，则返回错误
    if user.username == new_user.username || user.email == new_user.email {
        return Err(EveryError::AuthenticationError("用户名或邮箱已经存在".to_string()));
    }
    let new_user_with_uuid = UserCreateWithUuid{
        uuid: Uuid::new_v4().to_string(),
        username: new_user.username,
        password: new_user.password,
        email: new_user.email,
    };
    let user_uuid = new_user_with_uuid.uuid.clone();

    diesel::insert_into(users_table::table)
        .values(&new_user_with_uuid)
        .execute(pool)?;
    
    let inserted_user: UserDetail = users_table::table
        .filter(users_table::uuid.eq(&user_uuid))
        .first(pool)?;

    Ok(inserted_user)
}

pub async fn login_query_sql(pool: &mut MysqlConnection, user_query: UserQuery) -> Result<UserDetail, EveryError> {
    use crate::schema::users_table;
    use diesel::prelude::*;
    let username = user_query.username.unwrap_or_else(|| String::from(""));
    let email = user_query.email.unwrap_or_else(|| String::from(""));
    match (username.len(), email.len()) {
        (0, 0) => {
            return Err(EveryError::ValidationError("用户名和邮箱不能同时为空".to_string()));
        }
        _ => {}
    }
    match user_query.password {
        Some(password) => {
            if password.len() == 0 {
                return Err(EveryError::ValidationError("密码不能为空".to_string()));
            }else {
                //通过用户名和密码查询用户，如果用户名和密码匹配或者邮箱和密码匹配，则返回登录成功，否则返回登录失败
                let user: UserDetail = users_table::table
                //必须满足 users_table::username 等于 &username 或者 users_table::email 等于 &email
                .filter(users_table::username.eq(&username).or(users_table::email.eq(&email)))
                .filter(users_table::password.eq(&password))
                .first(pool)?;
            Ok(user)
            }
        }
        None => {
            return Err(EveryError::ValidationError("密码不能为空".to_string()));
        }
    }

    
}

pub async fn delete_user_sql(pool:&mut MysqlConnection,user_uuid:String)->Result<String,EveryError>{
    use crate::schema::users_table;
    use diesel::prelude::*;
    diesel::delete(users_table::table.filter(users_table::uuid.eq(&user_uuid))).execute(pool)?;
    Ok("删除成功".to_string())
}