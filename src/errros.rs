use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error};
use std::convert::From;
use uuid::ParseError;

#[derive(Fail, Debug)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error")]
    InternalServerError,

    #[fail(display = "BadRequest: {})", _0)]
    BadRequest(String),

    #[fail(display = "Unauthroized")]
    Unauthorized,
}
impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match *self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Service Error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
        }
    }
}
