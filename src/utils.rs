use crate::errors::EveryError;
use serde::Deserialize;
use sqlx::MySql;
use serde;

use serde::de::{self, Deserializer};
use regex::Regex;

pub fn deserialize_email<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    let v: String = String::deserialize(deserializer)?;
    let email_regex = Regex::new(r"^[a-zA-Z0-9_.+-]+@[a-zA-Z0-9-]+\.[a-zA-Z0-9-.]+$").unwrap();
    if email_regex.is_match(&v) {
        Ok(v)
    } else {
        Err(de::Error::custom("Invalid email address"))
    }
}

//暂未使用
pub async fn deserialize_is_exist<'d, D>(table_name: &str, field_name: &str, deserializer: D, pool: &sqlx::Pool<MySql>) -> Result<bool, EveryError>
where
    D: serde::Deserializer<'d>,
{
    // 反序列化
    let value: Result<String, D::Error> = Deserialize::deserialize(deserializer);
    
    let value = match value {
        Ok(val) => val,
        Err(e) => panic!("Error deserializing value: {:?}", e),
    };

    let row: (i64,) = sqlx::query_as(&format!("SELECT COUNT(*) FROM {} WHERE {} = ?", table_name, field_name))
        .bind(value)
        .fetch_one(pool)
        .await?;

    Ok(row.0 > 0)
}