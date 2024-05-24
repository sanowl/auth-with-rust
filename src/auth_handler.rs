use actix::{Handler, Message};
use actix_web::{dev::Payload, web, Error, FromRequest, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures::future::{ok, Ready};

use crate::{
    errors::ServiceError,
    models::{SlimUser, User},
    utils::decode_token,
    DbExecutor,
};
use bcrypt::verify;

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

impl Message for AuthData {
    type Result = Result<SlimUser, ServiceError>;
}

impl Handler<AuthData> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;

    fn handle(&mut self, msg: AuthData, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::*;

        let conn: &PgConnection = &self.0.get().unwrap();
        let mismatch_error = ServiceError::BadRequest("Invalid credentials".into());

        let maybe_user = users
            .filter(email.eq(&msg.email))
            .first::<User>(conn)
            .optional()?;

        if let Some(user) = maybe_user {
            if verify(&msg.password, &user.password)? {
                Ok(user.into())
            } else {
                Err(mismatch_error)
            }
        } else {
            Err(mismatch_error)
        }
    }
}

// Alias for clarity
pub type LoggedUser = SlimUser;

impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Ready<Result<LoggedUser, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let result = match req.cookie("auth-cookie") {
            Some(cookie) => decode_token(cookie.value())
                .map_err(|_| ServiceError::Unauthorized)
                .map(|user| user as LoggedUser),
            None => Err(ServiceError::Unauthorized),
        };

        ok(result.map_err(actix_web::Error::from))
    }
}
