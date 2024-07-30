use chrono::NaiveDateTime;
use diesel::prelude::{Queryable,Selectable,Insertable,QueryableByName};
use serde::{Serialize,Deserialize};
use crate::schema::{order_detail_table,order_status_table,order_table};
use bigdecimal::BigDecimal;

#[derive(Debug,Queryable, Insertable)]
#[table_name="order_table"]
pub struct Order {
    pub id: i32,
    pub user_uuid: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
#[derive(Debug,Queryable, Insertable)]
#[table_name="order_table"]
pub struct OrderCreate {
    pub user_uuid: String,
}



#[derive(Debug,Queryable, Insertable)]
#[table_name="order_detail_table"]
pub struct OrderDetail {
    pub id: i32,
    pub order_id: i32,
    pub product_id: i32,
    pub product_quantity: i32,
    pub unit_price: BigDecimal,
    pub order_status_id: i32,
}
#[derive(Debug,Queryable, Insertable)]
#[table_name="order_detail_table"]
pub struct OrderDetailCreate {
    pub order_id: i32,
    pub product_id: i32,
    pub product_quantity: i32,
    pub unit_price: BigDecimal,
    pub order_status_id: i32,
}

/*
1,pending_payment,待付款
2,paid,已付款
3,shipped,已发货
4,completed,已完成
5,cancelled,已取消
6,refunded,已退款

*/
#[derive(Queryable, Insertable)]
#[table_name="order_status_table"]
pub struct OrderStatus {
    pub id: i32,
    pub status_name: String,
    pub status_description: String,
}








