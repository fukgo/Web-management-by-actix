use crate::schema::user_profile_table::{self, avatar_url};
use diesel::prelude::{Queryable,Selectable,Insertable,QueryableByName};
use serde::{Deserialize, Serialize};
#[derive(Debug,Serialize,Deserialize,Clone,Queryable,Selectable)]
#[diesel(table_name = user_profile_table)]
pub struct ProfileDetail{
    pub user_uuid: String,
    pub real_name: Option<String>,
    pub bio: Option<String>,
    pub avatar_url: Option<String>,
    pub gender: Option<String>,
    pub age: Option<i32>,
}
#[derive(Debug,Serialize,Insertable)]
#[diesel(table_name = user_profile_table)]
pub struct ProfileCreate{
    pub user_uuid: String,
}
