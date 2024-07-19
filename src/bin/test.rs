use actix_web::{test,web,App,HttpResponse,http::StatusCode};
use reqwest;
use std::collections::HashMap;
use reqwest::header::HeaderMap;
pub async fn register_post()->Result<String,reqwest::Error>{
    let client = reqwest::Client::new();
    let mut header = HeaderMap::new();
    header.insert("Content-Type","application/json".parse().unwrap());
    //组装要提交的数据
    let mut data = HashMap::new();
    data.insert("username".to_string(), "test".to_string());
    data.insert("password".to_string(), "test".to_string());
    data.insert("email".to_string(), "aaa@aaa.com".to_string());
    let url = "http://127.0.0.1:8080/user/register";
    Ok(client.post(url).headers(header).json(&data).send().await?.text().await?)
}
pub async fn login_post()->Result<String,reqwest::Error>{
    let client = reqwest::Client::new();
    let mut header = HeaderMap::new();
    header.insert("Content-Type","application/json".parse().unwrap());
    
    let mut data = HashMap::new();
    data.insert("username".to_string(), "test".to_string());
    data.insert("password".to_string(), "test".to_string());
    let url = "http://127.0.0.1:8080/user/login";
    Ok(client.post(url).headers(header).json(&data).send().await?.text().await?)
}   
#[tokio::main]
async fn main() {
    if let Ok(resp) = register_post().await {
        println!("{:#?}", resp);
    }

    if let Ok(res) = login_post().await {
        println!("{:#?}", res);
    }
}

