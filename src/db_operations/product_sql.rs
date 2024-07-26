
use diesel::mysql::Mysql;
use diesel::sql_types::Bool;
use uuid::Uuid;
use crate::errors::EveryError;
use crate::models::product_model::*;
use diesel::MysqlConnection;


pub async fn get_product_detail_sql(pool: &mut MysqlConnection, product_id: i32) -> Result<ProductDetail, EveryError> {
    use crate::schema::product_table;
    use diesel::prelude::*;
    let user: ProductDetail = product_table::table
        .filter(product_table::id.eq(&product_id))
        .first(pool)?;
    Ok(user)
}



pub async fn post_new_product_sql(pool: &mut MysqlConnection, new_product: ProductCreate) -> Result<ProductDetail, EveryError> {
    use crate::schema::product_table;
    use diesel::prelude::*;
    diesel::insert_into(product_table::table)
        .values(&new_product)
        .execute(pool)?;
    let inserted_user: ProductDetail = product_table::table
        .filter(product_table::product_name.eq(&new_product.product_name))
        .first(pool)?;
    
    Ok(inserted_user)
        

}


pub async fn delete_product_sql(pool:&mut MysqlConnection,priduct_id:i32)->Result<String,EveryError>{
    use crate::schema::product_table;
    use diesel::prelude::*;
    diesel::delete(product_table::table.filter(product_table::id.eq(&priduct_id))).execute(pool)?;
    Ok("删除成功".to_string())
}
pub async fn get_all_product_types_sql(pool:&mut MysqlConnection)->Result<Vec<ProductTypesDetail>,EveryError>{
    use crate::schema::product_types_table;
    use diesel::prelude::*;
    let all_types: Vec<ProductTypesDetail> = product_types_table::table.load(pool)?;
    Ok(all_types)
}

