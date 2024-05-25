use crate::errors::ServiceError;
use crate::models::SlimUser;
use bcrypt::{hash, DEFAULT_COST};
use chrono::{Duration, Local};
use jwt::{decode, encode, Header, Validation};
use std::convert::From;
use std::env;

// Function to hash a plain password using bcrypt
pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    // Get the hashing cost from the environment variable or use the default cost
    let hashing_cost: u32 = match env::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    // Hash the password using the specified cost and return the result or an error
    hash(plain, hashing_cost).map_err(|_| ServiceError::InternalServerError)
}

// Claims struct represents the data stored in a JWT token
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String, // Issuer of the token
    sub: String, // Subject of the token
    iat: i64,    // Issued at timestamp
    exp: i64,    // Expiration timestamp
    email: String, // User email
}

impl Claims {
    // Constructor function to create a new Claims instance with the given email
    fn with_email(email: &str) -> Self {
        Claims {
            iss: "localhost".into(),
            sub: "auth".into(),
            email: email.to_owned(),
            iat: Local::now().timestamp(),
            exp: (Local::now() + Duration::hours(24)).timestamp(),
        }
    }
}

// Implement the From trait to convert Claims to SlimUser
impl From<Claims> for SlimUser {
    fn from(claims: Claims) -> Self {
        SlimUser {
            email: claims.email,
        }
    }
}

// Function to create a new JWT token from a SlimUser instance
pub fn create_token(data: &SlimUser) -> Result<String, ServiceError> {
    let claims = Claims::with_email(data.email.as_str());
    encode(&Header::default(), &claims, get_secret().as_ref())
        .map_err(|_err| ServiceError::InternalServerError)
}

// Function to decode a JWT token and extract the SlimUser data
pub fn decode_token(token: &str) -> Result<SlimUser, ServiceError> {
    decode::<Claims>(token, get_secret().as_ref(), &Validation::default())
        .map(|data| Ok(data.claims.into()))
        .map_err(|_err| ServiceError::Unauthorized)?
}

// Function to retrieve the JWT secret from an environment variable or use a default value
fn get_secret() -> String {
    env::var("JWT_SECRET").unwrap_or("my secret".into())
}