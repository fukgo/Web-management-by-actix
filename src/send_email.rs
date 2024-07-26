use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::error::Error;
use rand::Rng;
use redis::Commands;
use crate::errors::EveryError;
use std::time::Duration;
use actix_web::{web, HttpResponse, Responder};
use r2d2_redis::RedisConnectionManager;
use diesel::r2d2::Pool;
pub struct Email {
    pub from:String,
    pub to:String,
    pub subject:String,
    pub body:String,
}
pub struct EmailConfig {
    pub smtp_server: String,
    pub smtp_username: String,
    pub smtp_password: String,
}


impl Email{
    pub fn new(to: String, subject: String, body: String) -> Self {
        Email {
            from: "15271039530@163.com".to_string(),
            to,
            subject,
            body,
        }
    }


}
impl EmailConfig{
    pub fn new(smtp_server: String, smtp_username: String, smtp_password: String) -> Self {
        EmailConfig {
            smtp_server,
            smtp_username,
            smtp_password,
        }
    }
}

async fn generate_random_string() -> String {
    let mut rng = rand::thread_rng();
    let code: u32 = rng.gen_range(100000..=999999);
    code.to_string()
}

// pub fn send_email(email_config:EmailConfig, email: Email) -> Result<(), Box<dyn Error>> {
//     let creds = Credentials::new(email_config.smtp_username.to_string(), email_config.smtp_password.to_string());
//     let mailer = SmtpTransport::relay(&email_config.smtp_server)?
//         .credentials(creds)
//         .build();
//     let sent_email = Message::builder()
//         .from(email.from.parse()?)
//         .to(email.to.parse()?)
//         .subject(email.subject)
//         .body(email.body)?;
//     mailer.send(&sent_email)?;
//     Ok(())
// }
// 将验证码保存到 Redis
pub async fn save_code_to_redis(redis_conn: &mut redis::Connection, user_id: String, code: String) -> redis::RedisResult<()> {
    let _: () = redis_conn.set_ex(user_id, code, 180)?; // 设置过期时间为 180 秒（3 分钟）
    Ok(())
}


pub async fn send_validate_code(
    send_email: String,
    user_uuid: String,
    redis_pool: web::Data<Pool<RedisConnectionManager>>,
) -> Result<String, EveryError> {
    let code = generate_random_string().await;
    let email = Email::new(send_email.clone(), "验证码".to_string(), format!("您的验证码是：{}", code));
    let email_config = EmailConfig::new("smtp.163.com".to_string(), "your_username".to_string(), "your_password".to_string());
    let creds = Credentials::new(email_config.smtp_username.clone(), email_config.smtp_password.clone());

    let mailer = SmtpTransport::relay(&email_config.smtp_server)
        .map_err(|e| EveryError::EmailError(e.to_string()))?
        .credentials(creds)
        .build();

    let sent_email = Message::builder()
        .from(email.from.parse().map_err(|e: lettre::address::AddressError| EveryError::EmailError(e.to_string()))?)
        .to(email.to.parse().map_err(|e: lettre::address::AddressError| EveryError::EmailError(e.to_string()))?)
        .subject(email.subject)
        .body(email.body)
        .map_err(|e| EveryError::EmailError(e.to_string()))?;

    mailer.send(&sent_email).map_err(|e| EveryError::EmailError(e.to_string()))?;

    let mut redis_conn = redis_pool.get().map_err(|e| EveryError::DatabaseError(e.to_string()))?;

    redis_conn.set_ex(&user_uuid, &code, 600).map_err(|e| EveryError::DatabaseError(e.to_string()))?;

    Ok("验证码已发送".to_string())
}