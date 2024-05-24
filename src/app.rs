use crate::auth_routes::{get_me, login, logout};
use crate::invitation_routes::register_email;
use crate::models::DbExecutor;
use crate::register_routes::register_user;
use actix::prelude::*;
use actix_web::middleware::identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, App, HttpServer}; // Added web import
use diesel::r2d2::{ConnectionManager, Pool}; // Added r2d2 import
use diesel::PgConnection; // Added PgConnection import
use dotenv::dotenv; // Added dotenv import
