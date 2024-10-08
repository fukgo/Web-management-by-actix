
use chrono::NaiveDateTime;
use diesel::deserialize::FromSqlRow;
/*
Queryable: 这个 trait 用于从数据库查询返回的行中加载数据。当你执行 load、get_result、first 等方法时，Diesel 会尝试将返回的每一行数据转换为实现了 Queryable trait 的类型。

Selectable: 这个 trait 是 Diesel 1.4.0 新增的，用于更细粒度地控制如何从数据库查询返回的行中加载数据。与 Queryable 不同，Selectable 允许你选择性地加载某些字段，而不是必须加载所有字段。

Insertable: 这个 trait 用于将数据插入到数据库中。当你执行 insert_into 方法时，Diesel 会尝试将提供的数据转换为实现了 Insertable trait 的类型，然后将其插入到数据库中。

QueryableByName: 这个 trait 用于从数据库查询返回的行中加载数据，但与 Queryable 不同的是，QueryableByName 不需要知道查询返回的行的顺序，它可以根据字段的名称来加载数据。这在处理 SQL 查询的结果时特别有用，因为 SQL 查询的结果可能不是一个固定的结构。
 */
use diesel::prelude::{Queryable,Selectable,Insertable,QueryableByName};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use crate::schema::{users_table,roles_table,user_roles_table_correlation};
use crate::errors::EveryError;
#[derive(Debug,Serialize,Clone,Queryable)]
#[diesel(table_name = users_table)]
pub struct UserDetail{
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub user_status_id: i32,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    
}
#[derive(Debug,Deserialize,Clone,Insertable)]
#[diesel(table_name = users_table)]
pub struct UserCreate{
    pub username: String,
    #[serde(deserialize_with="crate::utils::deserialize_email")]
    pub email: String,
    pub password: String,

}
#[derive(Insertable)]
#[diesel(table_name = users_table)]
pub struct UserCreateAll {
    pub uuid: String,
    pub user_status_id:i32,
    pub username: String,
    pub password: String,
    pub email: String,
}
#[derive(Debug,Clone,Deserialize)]
pub struct UserLogin{
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,

}
use regex::Regex;

#[derive(Insertable)]
#[diesel(table_name = user_roles_table_correlation)]
pub struct UserRolesCorrelationCreate{
    pub user_uuid: String,
    pub role_id: i32,
}


