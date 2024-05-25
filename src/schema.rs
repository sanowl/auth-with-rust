table! {
  invitations (id) {
      id -> Uuid,
      email -> Varchar,
      expires_at -> Timestamp,
      created_at -> Timestamp,
      updated_at -> Nullable<Timestamp>,
      status -> InvitationStatus,
      user_id -> Nullable<Uuid>,
  }
}

table! {
  users (id) {
      id -> Uuid,
      email -> Varchar,
      password -> Varchar,
      created_at -> Timestamp,
      updated_at -> Nullable<Timestamp>,
      last_login_at -> Nullable<Timestamp>,
      is_active -> Bool,
      role -> UserRole,
      profile_image -> Nullable<Varchar>,
  }
}

table! {
  user_invitations (user_id, invitation_id) {
      user_id -> Uuid,
      invitation_id -> Uuid,
      accepted_at -> Nullable<Timestamp>,
      created_at -> Timestamp,
      updated_at -> Nullable<Timestamp>,
  }
}

joinable!(invitations -> users (user_id));
joinable!(user_invitations -> invitations (invitation_id));
joinable!(user_invitations -> users (user_id));

allow_tables_to_appear_in_same_query!(
  invitations,
  users,
  user_invitations,
);