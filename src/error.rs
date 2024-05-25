// Import necessary modules
use actix::{Handler, Message};
use actix_web::{FromRequest, HttpRequest, middleware::identity::RequestIdentity};
use diesel::prelude::*;
use bcrypt::verify;
use crate::models::{DbExecutor, User, SlimUser};
use crate::errors::ServiceError;
use crate::utils::decode_token;

// Define a struct to hold authentication data
#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub email: String,
    pub password: String,
}

// Implement the Message trait for AuthData
impl Message for AuthData {
    // The result of handling an AuthData message is a SlimUser or a ServiceError
    type Result = Result<SlimUser, ServiceError>;
}

// Implement the Handler trait for DbExecutor to handle AuthData messages
impl Handler<AuthData> for DbExecutor {
    // The result of handling an AuthData message is a SlimUser or a ServiceError
    type Result = Result<SlimUser, ServiceError>;

    // Handle an AuthData message
    fn handle(&mut self, msg: AuthData, _: &mut Self::Context) -> Self::Result {
        // Get a reference to the database connection
        let conn: &PgConnection = &self.0.get().unwrap();
        
        // Find a user by email
        let user = find_user_by_email(conn, &msg.email)?;
        
        // Verify the password
        verify_password(&msg.password, &user.password)?;
        
        // Return the user as a SlimUser
        Ok(user.into())
    }
}

// Find a user by email
fn find_user_by_email(conn: &PgConnection, email: &str) -> Result<User, ServiceError> {
    use crate::schema::users::dsl::{users, email};
    
    // Load users with the given email
    let mut items = users.filter(email.eq(email)).load::<User>(conn)?;
    
    // Return the first user, or a ServiceError if none are found
    items.pop().ok_or(ServiceError::BadRequest("User not found".into()))
}

// Verify a password
fn verify_password(password: &str, hashed_password: &str) -> Result<(), ServiceError> {
    match verify(password, hashed_password) {
        Ok(matching) => {
            if matching {
                Ok(())
            } else {
                Err(ServiceError::BadRequest("Username and Password don't match".into()))
            }
        }
        Err(_) => Err(ServiceError::BadRequest("Invalid password".into())),
    }
}

// Define a type alias for LoggedUser
pub type LoggedUser = SlimUser;

// Implement the FromRequest trait for LoggedUser
impl<S> FromRequest<S> for LoggedUser {
    type Config = ();
    type Result = Result<LoggedUser, ServiceError>;

    // Get a LoggedUser from a request
    fn from_request(req: &HttpRequest<S>, _: &Self::Config) -> Self::Result {
        if let Some(identity) = req.identity() {
            let user: SlimUser = decode_token(&identity)?;
            Ok(user as LoggedUser)
        } else {
            Err(ServiceError::Unauthorized)
        }
    }
}