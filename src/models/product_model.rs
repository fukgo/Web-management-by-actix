use chrono::NaiveDateTime;
use diesel::prelude::{Queryable,Selectable,Insertable,QueryableByName};
use serde::{Serialize,Deserialize};
use crate::schema::{product_table,product_types_table};
use bigdecimal::BigDecimal;
#[derive(Debug,Serialize,Deserialize,Clone,Queryable,Insertable)]
#[diesel(table_name = product_table)]
pub struct ProductCreate{
    pub product_type_id: i32,
    pub product_name :String,
    pub product_price: BigDecimal,
    pub product_stock: i32,
    #[serde(default)]
    pub product_description: Option<String>,
    #[serde(default)]
    pub product_icon: Option<String>,
    pub product_status_id: i32,
}
#[derive(Debug,Serialize,Clone,Queryable)]
#[diesel(table_name = product_table)]
pub struct ProductDetail {
    pub id: i32,
    pub product_type_id: i32,
    pub product_name: String,
    pub product_price: bigdecimal::BigDecimal,
    pub product_stock: i32,
    #[serde(default)]//如果在 JSON 中这两个字段不存在或者为空，serde 会将它们设置为 None。
    pub product_description: Option<String>,
    #[serde(default)]//如果在 JSON 中这两个字段不存在或者为空，serde 会将它们设置为 None。
    pub product_icon: Option<String>,
    pub product_status_id: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[derive(Debug,Serialize,Clone,Queryable)]
#[diesel(table_name = product_types_table)]
pub struct ProductTypesDetail{
    pub id: i32,
    pub type_name: String,
    #[serde(default)]
    pub type_icon: Option<String>,
    pub description: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug,Serialize,Clone,Insertable)]
#[diesel(table_name = product_types_table)]
pub struct ProductTypesCreate{
    pub type_name: String,
    pub description: String,
}

#[derive(Queryable,Serialize,Clone,Selectable)]
#[diesel(table_name = product_types_table)]
pub struct ProductTypeName{
    pub type_name:String
}

