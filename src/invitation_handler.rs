use crate::errros::ServiceError;
use crate::models::{DbExecutor, Invitation, SlimUser, User};
use crate::utils::hash_password;
use actix::{Handler, Message};
use chrono::Local;
use diesel::prelude::*;
use uuid::Uuid;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

#[derive(Debug)]
pub struct RegisterUser {
    pub invitation_id: String,
    pub password: String,
}

impl Message for RegisterUser {
    type Result = Result<SlimUser, ServiceError>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;

    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::{id, invitations};
        use crate::schema::users::dsl::users;

        let conn: &mut PgConnection = &mut self.0.get().unwrap();

        let invitation_id = Uuid::parse_str(&msg.invitation_id)?;

        invitations
            .filter(id.eq(invitation_id))
            .load::<Invitation>(conn)
            .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
            .and_then(|mut result| {
                if let Some(invitation) = result.pop() {
                    if invitation.expires_at > Local::now().naive_local() {
                        let password: String = hash_password(&msg.password)?;
                        let user = User::with_detail(invitation.email, password);
                        let inserted_user: User =
                            diesel::insert_into(users).values(&user).get_result(conn)?;
                        return Ok(inserted_user.into());
                    }
                }
                Err(ServiceError::BadRequest("Invalid Invitation".into()))
            })
    }
}