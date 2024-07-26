use crate::schema::user_profile_table;
use diesel::prelude::{Queryable,Selectable,Insertable,QueryableByName};
use serde::{Deserialize, Serialize};
#[derive(Debug,Serialize,Deserialize,Clone,Queryable,Selectable)]
#[diesel(table_name = user_profile_table)]
pub struct ProfileDetail{
    pub user_uuid: String,
    pub real_name: String,
    pub bio: String,
    pub avatar_url: String,
    pub gender: String,
    pub age: u32,
}
#[derive(Debug,Serialize,Insertable)]
#[diesel(table_name = user_profile_table)]
pub struct ProfileCreate{
    pub user_uuid: String,
}