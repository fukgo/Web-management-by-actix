use jsonwebtoken::{encode,EncodingKey,Header,Algorithm};
use serde::{Serialize,Deserialize};

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    //标识主题，也就是 token 的所有者。
    sub: String,
    //预定义的声明，表示 token 的过期时间。这通常是一个 Unix 时间戳，表示从 1970 年 1 月 1 日 00:00:00 UTC 到 token 过期时的秒数。
    exp: usize,
}

pub fn generate_token(user_uuid: String) -> String {
    //创建一个 Claims 结构体实例，设置 sub 字段为 user_id，exp 字段为当前时间戳加上 24 小时的秒数。
    let my_claims = Claims { sub: user_uuid, exp: 24 * 60 * 60 + chrono::Utc::now().timestamp() as usize };
    //创建一个 EncodingKey 实例，用于加密 token。
    let key = b"secret";
    //创建一个 Header 实例，用于设置 token 的头部信息。
    let header = Header::new(Algorithm::HS256);
    //调用 encode 函数，将 Claims 结构体实例 my_claims 编码成一个 token 字符串。
    encode(&header, &my_claims, &EncodingKey::from_secret(key)).unwrap()
}