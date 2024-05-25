use crate::errors::ServiceError;
use crate::models::{DbExecutor, Invitation, SlimUser, User};
use crate::utils::hash_password;
use actix::{Handler, Message};
use chrono::Local;
use diesel::prelude::*;
use uuid::Uuid;
use serde::Deserialize;

// Struct representing the user data received in the request
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

// Message struct for registering a user
#[derive(Debug)]
pub struct RegisterUser {
    pub invitation_id: String,
    pub password: String,
}

// Implement the Message trait for RegisterUser
// This defines the result type of the message handling
impl Message for RegisterUser {
    type Result = Result<SlimUser, ServiceError>;
}

// Implement the Handler trait for DbExecutor
// This defines how the message is handled by the DbExecutor actor
impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;

    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::{id, invitations};
        use crate::schema::users::dsl::users;

        // Get a database connection from the connection pool
        let conn: &mut PgConnection = &mut self.0.get().unwrap();

        // Parse the invitation ID from the message
        let invitation_id = Uuid::parse_str(&msg.invitation_id)?;

        // Find the invitation in the database
        invitations
            .filter(id.eq(invitation_id))
            .load::<Invitation>(conn)
            .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
            .and_then(|mut result| {
                // If the invitation is found and not expired
                if let Some(invitation) = result.pop() {
                    if invitation.expires_at > Local::now().naive_local() {
                        // Hash the password
                        let password: String = hash_password(&msg.password)?;

                        // Create a new user with the invitation email and hashed password
                        let user = User::with_detail(invitation.email, password);

                        // Insert the new user into the database
                        let inserted_user: User =
                            diesel::insert_into(users).values(&user).get_result(conn)?;

                        // Return the inserted user as a SlimUser
                        return Ok(inserted_user.into());
                    }
                }
                // If the invitation is invalid or expired, return an error
                Err(ServiceError::BadRequest("Invalid Invitation".into()))
            })
    }
}