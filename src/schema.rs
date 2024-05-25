table! {
  invitations (id) {
      id -> Uuid,                         // Primary key of the invitations table
      email -> Varchar,                   // Email address associated with the invitation
      expires_at -> Timestamp,            // Timestamp indicating when the invitation expires
      created_at -> Timestamp,            // Timestamp of when the invitation was created
      updated_at -> Nullable<Timestamp>,  // Optional timestamp of when the invitation was last updated
      status -> InvitationStatus,         // Status of the invitation (e.g., pending, accepted, declined)
      user_id -> Nullable<Uuid>,          // Optional foreign key referencing the users table
  }
}

table! {
  users (id) {
      id -> Uuid,                         // Primary key of the users table
      email -> Varchar,                   // Email address of the user
      password -> Varchar,                // Hashed password of the user
      created_at -> Timestamp,            // Timestamp of when the user was created
      updated_at -> Nullable<Timestamp>,  // Optional timestamp of when the user was last updated
      last_login_at -> Nullable<Timestamp>,   // Optional timestamp of the user's last login
      is_active -> Bool,                  // Flag indicating if the user account is active
      role -> UserRole,                   // Role or permission level of the user (e.g., admin, regular user)
      profile_image -> Nullable<Varchar>, // Optional URL or path to the user's profile image
  }
}

table! {
  user_invitations (user_id, invitation_id) {
      user_id -> Uuid,                    // Foreign key referencing the users table
      invitation_id -> Uuid,              // Foreign key referencing the invitations table
      accepted_at -> Nullable<Timestamp>, // Optional timestamp of when the user accepted the invitation
      created_at -> Timestamp,            // Timestamp of when the user-invitation relationship was created
      updated_at -> Nullable<Timestamp>,  // Optional timestamp of when the user-invitation relationship was last updated
  }
}

joinable!(invitations -> users (user_id));
// Specifies that the invitations table has a foreign key user_id referencing the users table

joinable!(user_invitations -> invitations (invitation_id));
// Specifies that the user_invitations table has a foreign key invitation_id referencing the invitations table

joinable!(user_invitations -> users (user_id));
// Specifies that the user_invitations table has a foreign key user_id referencing the users table

allow_tables_to_appear_in_same_query!(
  invitations,
  users,
  user_invitations,
);
// Allows the invitations, users, and user_invitations tables to be used together in the same query
// This is necessary when performing joins or complex queries involving multiple tables