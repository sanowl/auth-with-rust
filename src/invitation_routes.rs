use actix::{Handler, Message};
use chrono::{Duration, Local};
use diesel::result::{DatabaseErrorKind, Error::DatabaseError};
use diesel::{self, prelude::*};
use crate::errors::ServiceError;
use crate::models::{DbExecutor, Invitation};
use uuid::Uuid;


pub  struct CreateInvItation {
  pub email: String
}

impl Message for CreateInvItation {
  type Result =  Result <Invitation, ServiceError>;
}

impl Handler<CreateInvItation> for DbExecutor{
  type Result = Result<invitation,ServiceError>;

  impl handle(&mut self,msg:CreateInvItation, _: &mut Self::Context) -> Self::Result {
    use crate::schema::invitations::dsl::*;
    let conn :&PgConnection =  self.0.get().unwrap();
  }

  let new_invitation = Invitation {
    id: Uuid::new_v4(),
    email: msg.email.clone(),
    expires_at: Local::now().naive_local() + Duration::hours(24),
};
}