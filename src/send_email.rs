use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::error::Error;
use rand::Rng;
use redis::Commands;
use std::time::Duration;
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

fn generate_random_string(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let code: u32 = rng.gen_range(100000,999999);
    code.to_string()
}

pub fn send_email(email_config:EmailConfig, email: Email) -> Result<(), Box<dyn Error>> {
    let creds = Credentials::new(email_config.smtp_username.to_string(), email_config.smtp_password.to_string());
    let mailer = SmtpTransport::relay(&email_config.smtp_server)?
        .credentials(creds)
        .build();
    let sent_email = Message::builder()
        .from(email.from.parse()?)
        .to(email.to.parse()?)
        .subject(email.subject)
        .body(email.body)?;
    mailer.send(&sent_email)?;
    Ok(())
}
// 将验证码保存到 Redis
pub fn save_code_to_redis(redis_conn: &mut redis::Connection, user_id: &str, code: &str) -> redis::RedisResult<()> {
    let _: () = redis_conn.set_ex(user_id, code, 180)?; // 设置过期时间为 180 秒（3 分钟）
    Ok(())
}
