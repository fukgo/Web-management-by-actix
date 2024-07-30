


use captcha::Captcha;
use captcha::filters::{Noise, Wave, Dots};
use std::path::Path;
use chrono_tz::Asia::Shanghai;
use chrono::Utc;
fn generate_code(day: String, time: String) -> String {
    let mut captcha = Captcha::new();
    captcha.add_chars(7); // 增加字符数量
    let captcha_chars = captcha.chars_as_string();

    let path = format!("static/code_img/{}/{}.png", day, time);
    captcha
    .apply_filter(Noise::new(0.4))
    .apply_filter(Wave::new(2.0, 20.0).horizontal())
    .apply_filter(Wave::new(2.0, 20.0).vertical())
    .view(220, 120)
    .apply_filter(Dots::new(15))
    .save(Path::new(&path))
    .expect("save failed");

    captcha_chars
}
fn main() {
    let date_time = Utc::now().with_timezone(&Shanghai);
    let day = date_time.format("%Y-%m-%d").to_string();
    let time = date_time.format("%H:%M:%S").to_string();
    println!("{},{}",day,time);
    let captcha_chars =  generate_code(day,time);
    println!("Captcha characters: {}", captcha_chars);



}