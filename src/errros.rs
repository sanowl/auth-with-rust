use actix_web::{error::ResponseError, HttpResponse};
use diesel::result::{DatabaseErrorKind, Error};
use std::convert::From;
use uuid::ParseError;
