use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error};
use std::convert::From;
use uuid::Error as UuidError;

#[derive(Debug)]
pub enum ServiceError {
    InternalServerError,
    BadRequest(String),
    Unauthorized,
    NotFound,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServiceError::InternalServerError => {
                HttpResponse::InternalServerError().json("Internal Server Error")
            }
            ServiceError::BadRequest(ref message) => HttpResponse::BadRequest().json(message),
            ServiceError::Unauthorized => HttpResponse::Unauthorized().json("Unauthorized"),
            ServiceError::NotFound => HttpResponse::NotFound().json("Not Found"),
        }
    }
}

impl From<UuidError> for ServiceError {
    fn from(_: UuidError) -> ServiceError {
        ServiceError::BadRequest("Invalid UUID".into())
    }
}

impl From<Error> for ServiceError {
    fn from(error: Error) -> ServiceError {
        match error {
            Error::DatabaseError(kind, info) => {
                match kind {
                    DatabaseErrorKind::UniqueViolation => {
                        let message = info.details().unwrap_or_else(|| info.message()).to_string();
                        ServiceError::BadRequest(message)
                    }
                    DatabaseErrorKind::NotFound => {
                        ServiceError::NotFound
                    }
                    _ => ServiceError::InternalServerError,
                }
            }
            _ => ServiceError::InternalServerError,
        }
    }
}