
use uuid::Uuid;
use crate::errors::EveryError;
use crate::models::user_model::{UserDetail,UserCreate,UserLogin,UserCreateAll,UserRolesCorrelationCreate};
use crate::models::profile_model::{ProfileCreate,ProfileDetail};
use crate::schema::user_roles_table_correlation;
use crate::schema::users_table;
use diesel::MysqlConnection;

pub async fn post_new_user_sql(pool: &mut MysqlConnection, new_user: UserCreate) -> Result<UserDetail, EveryError> {
    use crate::schema::users_table;
    use diesel::prelude::*;
    //查询用户名和邮箱是否已经存在
    let user: Option<UserDetail> = users_table::table
        .filter(users_table::username.eq(&new_user.username).or(users_table::email.eq(&new_user.email)))
        .get_result(pool)
        .optional()?;
    match user {
        Some(_) => {
            return Err(EveryError::AuthenticationError("用户名或邮箱已经存在".to_string()));
        },
        None => {
            let new_user_with_uuid = UserCreateAll{
                uuid: Uuid::new_v4().to_string(),
                user_status_id: 1,
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
        },
    }
}

pub async fn login_query_sql(pool: &mut MysqlConnection, user_login: UserLogin) -> Result<UserDetail, EveryError> {
    use crate::schema::{users_table,user_profile_table};
    use diesel::prelude::*;
    if user_login.all_fields_present()==false{
        return Err(EveryError::ValidationError("输入字段错误".to_string()));
    }
    if user_login.is_valid_email()==false{
        return Err(EveryError::ValidationError("邮箱格式错误".to_string()));
    }
    let username = user_login.username.clone().unwrap();
    let email = user_login.email.clone().unwrap();
    let password = user_login.password.clone().unwrap();
    let user:Option<UserDetail> = users_table::table
        .filter(users_table::username.eq(&username).or(users_table::email.eq(&email)))
        .filter(users_table::password.eq(&password))
        .get_result(pool)
        .optional()?;

    match user {
        Some(user) => {
            let user_profile = ProfileCreate {
                user_uuid: user.uuid.clone(),
            };
            //创建用户的同时创建profile
            diesel::insert_into(user_profile_table::table)
                .values(&user_profile)
                .execute(pool)?;

            let user_roles_correlation = UserRolesCorrelationCreate {
                user_uuid: user.uuid.clone(),
                role_id: 3,
            };
            //创建用户的同时，为一个用户添加一个角色
            diesel::insert_into(user_roles_table_correlation::table)
                .values(&user_roles_correlation)
                .execute(pool)?;

            Ok(user)
        }
        None => Err(EveryError::ValidationError("用户名或密码错误".to_string())),
    }
}
pub async fn delete_user_sql(pool:&mut MysqlConnection,user_uuid:String)->Result<String,EveryError>{
    use crate::schema::users_table;
    use diesel::prelude::*;
    diesel::delete(users_table::table.filter(users_table::uuid.eq(&user_uuid))).execute(pool)?;
    Ok("删除成功".to_string())
}
//
pub async fn valide_email(pool:&mut MysqlConnection,email:String)->Result<bool,EveryError>{
    use crate::schema::users_table;
    use diesel::prelude::*;
    let user:Option<UserDetail> = users_table::table
        .filter(users_table::email.eq(&email))
        .get_result(pool)
        .optional()?;
    match user {
        Some(_) => {
            Ok(true)
        },
        None => {
            Ok(false)
        },
    }
}