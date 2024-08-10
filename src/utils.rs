use crate::errors::EveryError;
use serde::Deserialize;
use sqlx::MySql;
use serde;
use actix_web::web;
use serde::de::{self, Deserializer};
use regex::Regex;
use captcha::Captcha;
use captcha::filters::{Noise, Wave, Dots};
use std::path::Path;
use chrono_tz::Asia::Shanghai;
use chrono::Utc;
use lazy_static::lazy_static;
use std::sync::Mutex;

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

pub async fn generate_code() -> (String,Vec<u8>) {
    let date_time = Utc::now().with_timezone(&Shanghai);
    let mut captcha = Captcha::new();
    captcha.add_chars(6); // 增加字符数量
    let captcha_chars = captcha.chars_as_string();

    captcha
        .apply_filter(Noise::new(0.4))
        .apply_filter(Wave::new(2.0, 20.0).horizontal())
        .apply_filter(Wave::new(2.0, 20.0).vertical())
        .view(220, 120)
        .apply_filter(Dots::new(15));

    let image_bytes = captcha.as_png().expect("Failed to generate PNG");
    (captcha_chars,image_bytes)
}