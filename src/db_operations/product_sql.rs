use crate::errors::EveryError;
use crate::models::product_model::*;
use crate::schema::{product_table, product_types_table};
use diesel::MysqlConnection;

pub async fn get_product_detail_sql(
    pool: &mut MysqlConnection,
    product_id: i32,
) -> Result<ProductDetail, EveryError> {
    use diesel::prelude::*;
    let user: ProductDetail = product_table::table
        .filter(product_table::id.eq(&product_id))
        .first(pool)?;
    Ok(user)
}

pub async fn post_product_type_sql(
    pool: &mut MysqlConnection,
    new_product_type: ProductTypesCreate,
) -> Result<ProductTypesDetail, EveryError> {
    use diesel::prelude::*;
    diesel::insert_into(product_types_table::table)
        .values(&new_product_type)
        .execute(pool)?;
    let inserted_type: ProductTypesDetail = product_types_table::table
        .filter(product_types_table::type_name.eq(&new_product_type.type_name))
        .first(pool)?;
    Ok(inserted_type)
}

pub async fn post_new_product_sql(
    pool: &mut MysqlConnection,
    new_product: ProductCreate,
) -> Result<ProductDetail, EveryError> {
    use diesel::prelude::*;
    diesel::insert_into(product_table::table)
        .values(&new_product)
        .execute(pool)?;
    let inserted_product: ProductDetail = product_table::table
        .filter(product_table::product_name.eq(&new_product.product_name))
        .first(pool)?;

    Ok(inserted_product)
}

pub async fn delete_product_sql(
    pool: &mut MysqlConnection,
    priduct_id: i32,
) -> Result<String, EveryError> {
    use diesel::prelude::*;
    diesel::delete(product_table::table.filter(product_table::id.eq(&priduct_id))).execute(pool)?;
    Ok("删除成功".to_string())
}
pub async fn get_all_product_types_sql(
    pool: &mut MysqlConnection,
) -> Result<Vec<ProductTypesDetail>, EveryError> {
    use diesel::prelude::*;
    let all_types: Vec<ProductTypesDetail> = product_types_table::table.load(pool)?;
    Ok(all_types)
}

//通过page获取

pub async fn get_all_product_by_list_sql(
    pool: &mut MysqlConnection,
    page: u32,
    page_size: u32,
    type_id:u32
) -> Result<(u32, Vec<ProductDetail>), EveryError> {
    use diesel::prelude::*;
    use diesel::dsl::count;
    //计算偏移量
    let offset = (page - 1) * page_size;

    // 获取总的产品数量
    let total_count: i64 = product_table::table
        .select(count(product_table::id))
        .first(pool)?;

    // 计算总页数
    let total_pages = (total_count as u32 + page_size - 1) / page_size;
    //返回product_type_id的产品
    let all_products: Vec<ProductDetail> = product_table::table
        .filter(product_table::product_type_id.eq(type_id as i32))
        .limit(page_size.into())
        .offset(offset.into())
        .load(pool)?;

    Ok((total_pages, all_products))
}
