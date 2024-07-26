use std::fmt;
use actix_web::{HttpResponse, error, http::StatusCode};
use serde::Serialize;
use serde_json::json;
#[derive(Debug)]
pub enum EveryError{
    NotFound(String),
    DatabaseError(String),
    ValidationError(String),
    AuthenticationError(String),
    ParseError(String),
    InternalError(String),
    ActixError(actix_web::Error),
    DieselError(diesel::result::Error),
    SessionError(actix_session::SessionInsertError),
    EmailError(String),
}

impl EveryError{
    fn error_response(&self)->String{
        match self{
            EveryError::NotFound(message) => {
                println!("Not Found Error: {}", message);
                "Not Found Error".to_string()
            }
            EveryError::DatabaseError(message) => {
                println!("Database Error: {}", message);
                "Database Error".to_string()
            }
            EveryError::ValidationError(message) => {
                println!("Validation Error: {}", message);
                "Validation Error".to_string()
            }
            EveryError::AuthenticationError(message) => {
                println!("Authentication Error: {}", message);
                "Authentication Error".to_string()
            }
            EveryError::ParseError(message) => {
                println!("Parse Error: {}", message);
                "Parse Error".to_string()
            }
            EveryError::InternalError(message) => {
                println!("Internal Error: {}", message);
                "Internal Error".to_string()
            }
            EveryError::ActixError(error) => {
                println!("Actix Error: {}", error);
                "Actix Error".to_string()
            }
            EveryError::DieselError(error) => {
                println!("Diesel Error: {}", error);
                "Diesel Error".to_string()
            }
            EveryError::SessionError(error) => {
                println!("Session Error: {}", error);
                "Session Error".to_string()
            }
            EveryError::EmailError(error) => {
                println!("Email Error: {}", error);
                "Email Error".to_string()
            }
        }
    }
}

impl error::ResponseError for EveryError{
    fn status_code(&self) -> StatusCode {
        match self {
            EveryError::NotFound(_) => StatusCode::NOT_FOUND,
            EveryError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::ValidationError(_) => StatusCode::BAD_REQUEST,
            EveryError::AuthenticationError(_) => StatusCode::UNAUTHORIZED,
            EveryError::ParseError(_) => StatusCode::BAD_REQUEST,
            EveryError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::ActixError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::DieselError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::SessionError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            EveryError::EmailError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> HttpResponse {
        let error_message = self.to_string();
        HttpResponse::build(self.status_code())
            .json(json!({ "error": error_message }))
    }
}

impl fmt::Display for EveryError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        match self{
            EveryError::NotFound(message) => write!(f, "Not Found: {}", message),
            EveryError::DatabaseError(message) => write!(f, "Database Error: {}", message),
            EveryError::ValidationError(message) => write!(f, "Validation Error: {}", message),
            EveryError::AuthenticationError(message) => write!(f, "Authentication Error: {}", message),
            EveryError::ParseError(message) => write!(f, "Parse Error: {}", message),
            EveryError::InternalError(message) => write!(f, "Internal Error: {}", message),
            EveryError::ActixError(error) => write!(f, "Actix Error: {}", error),
            EveryError::DieselError(error) => write!(f, "Diesel Error: {}", error),
            EveryError::SessionError(error) => write!(f, "Session Error: {}", error),
            EveryError::EmailError(error) => write!(f, "Email Error: {}", error),
        }
    }
}

impl From<actix_web::Error> for EveryError{
    fn from(error: actix_web::Error) -> Self{
        EveryError::ActixError(error)
    }
}
impl From<diesel::result::Error> for EveryError{
    fn from(error: diesel::result::Error) -> Self{
        EveryError::DieselError(error)
    }
}