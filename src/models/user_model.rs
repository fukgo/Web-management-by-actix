
use chrono::{naive::serde::ts_milliseconds::deserialize, NaiveDateTime};
use diesel::prelude::{Queryable,Selectable,Insertable};
use actix_web::web;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::schema::users_table;
use diesel::QueryableByName;
#[derive(Debug,Serialize,Deserialize,Clone,Queryable,Selectable,QueryableByName)]
#[diesel(table_name = users_table)]
pub struct UserDetail{
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub email: String,
    pub created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
#[derive(Debug,Serialize,Deserialize,Clone,Insertable,QueryableByName)]
#[diesel(table_name = users_table)]
pub struct UserCreate{
    pub username: String,
    pub password: String,
    pub email: String,
}
#[derive(Insertable)]
#[diesel(table_name = users_table)]
pub struct UserCreateWithUuid {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub email: String,
}
#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct UserQuery{
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
}



