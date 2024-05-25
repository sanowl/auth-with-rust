use actix_web::web::Json;
use actix_web::{AsyncResponder, FutureResponse, HttpResponse, HttpRequest, ResponseError, Json};
use actix_web::middleware::identity::RequestIdentity;
use futures::future::Future;
use crate::utils::create_token;
use crate::auth_handler::{AuthData, LoggedUser};
use crate::app::AppState;

pub fn login((auth_data, req): (Json<AuthData>, HttpRequest<AppState>)) -> FutureResponse<HttpResponse> {
    req.state()
    .db
}