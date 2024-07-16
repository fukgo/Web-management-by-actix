use std::fmt;
use actix_web::{HttpResponse, error, http::StatusCode};
use serde::Serialize;
use serde_json::json;
#[derive(Debug,Serialize)]
pub enum EveryError{
    NotFound,
    DatabaseError,
    ValidationError,
    AuthenticationError,
    ParseError,
    InternalError,//InternalError通常用于表示服务器内部错误
    ActixError,
    DieselError,
    OtherErr
}
#[derive(Debug, Serialize)]
pub struct EveryResponseError{
    pub error: String
}

//实现ResponseError trait
impl error::ResponseError for EveryError{
    fn status_code(&self) -> StatusCode {
        match self {
            EveryError::NotFound => StatusCode::NOT_FOUND,
            EveryError::DatabaseError => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::ValidationError => StatusCode::BAD_REQUEST,
            EveryError::AuthenticationError => StatusCode::UNAUTHORIZED,
            EveryError::ParseError => StatusCode::BAD_REQUEST,
            EveryError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::OtherErr => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::ActixError => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::DieselError => StatusCode::INTERNAL_SERVER_ERROR,
        
        }
    }
    fn error_response(&self) -> HttpResponse {
        //to_string方法使用了fmt::Display trait。
        let error_message = self.to_string(); // 使用fmt::Display trait将错误转换为字符串
        HttpResponse::build(self.status_code())
            .json(json!({ "error": error_message })) // 返回一个包含错误信息的JSON响应
    }
}


//实现fmt::Display trait
impl fmt::Display for EveryError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            EveryError::NotFound => write!(f, "Not Found"),
            EveryError::DatabaseError => write!(f, "Database Error"),
            EveryError::ValidationError => write!(f, "Validation Error"),
            EveryError::AuthenticationError => write!(f, "Authentication Error"),
            EveryError::ParseError => write!(f, "Parse Error"),
            EveryError::InternalError => write!(f, "Internal Error"),
            EveryError::OtherErr => write!(f, "Other Error"),
            EveryError::ActixError => write!(f, "Actix Error"),
            EveryError::DieselError => write!(f, "Diesel Error"),
        }
    }
}
//actix_web::Error可以转换为EveryError
impl From<actix_web::Error> for EveryError{
    fn from(_error: actix_web::Error) -> Self{
        EveryError::ActixError
    }
}
impl From<diesel::result::Error> for EveryError{
    fn from(_error: diesel::result::Error) -> Self{
        EveryError::DieselError
    }
}