
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
    #[serde(deserialize_with = "generate_uuid")]
    pub uuid: Option<String>,
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

fn generate_uuid<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let _ = String::deserialize(deserializer)?;
    Ok(Some(Uuid::new_v4().to_string()))
}


