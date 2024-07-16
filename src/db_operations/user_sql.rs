use uuid::Uuid;
use crate::errors::EveryError;
use crate::models::user_model::{UserDetail,UserCreate,UserQuery};
use diesel::MysqlConnection;
use actix_session::Session;
//post新用户，返回
pub async fn post_new_user_sql(pool: &mut MysqlConnection, new_user: UserCreate) -> Result<UserDetail, EveryError> {
    use crate::schema::users_table;
    use diesel::prelude::*;
    let user_uuid = new_user.uuid.as_ref().unwrap().clone();
    diesel::insert_into(users_table::table)
        .values(&new_user)
        .execute(pool)?;
    
    let inserted_user: UserDetail = users_table::table
        .filter(users_table::uuid.eq(&user_uuid))
        .first(pool)?;

    Ok(inserted_user)
}

pub async fn login_query_sql(pool: &mut MysqlConnection, user_query: UserQuery,session: Session,) -> Result<UserDetail, EveryError> {
    use crate::schema::users_table;
    use diesel::prelude::*;
    let username = user_query.username.unwrap_or_else(|| String::from(""));
    let email = user_query.email.unwrap_or_else(|| String::from(""));
    let password = user_query.password.unwrap();
    
    //通过用户名和密码查询用户，如果用户名和密码匹配或者邮箱和密码匹配，则返回登录成功，否则返回登录失败
    let user: UserDetail = users_table::table
    //必须满足 users_table::username 等于 &username 或者 users_table::email 等于 &email
        .filter(users_table::username.eq(&username).or(users_table::email.eq(&email)))
        .filter(users_table::password.eq(&password))
        .first(pool)?;
    match user {
        user =>{
            session.set("user_uuid", user.uuid);
            return Ok(user);
        },
        _ => {
            return Err(EveryError::AuthenticationError("用户名或密码错误".to_string()));
        }

    }
}
