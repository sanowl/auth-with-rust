use actix_web::{AsyncResponder, HttpResponse, Json, Path, ResponseError, State, web};
use futures::future::Future;
use crate::app::AppState;
use crate::register_handler::{RegisterUser, UserData};

pub async fn register_user(
    path: Path<String>,
    user_data: Json<UserData>,
    state: State<AppState>,
) -> impl ResponseError<Error = actix_web::Error> {
    // Extract the invitation_id from the path parameter
    // Extract the user data (password) from the JSON payload
    let msg = RegisterUser {
        invitation_id: path.into_inner(),
        password: user_data.password.clone(),
    };

    // Send the RegisterUser message to the database actor
    // and await the response
    state.db.send(msg).await.map(|db_response| {
        match db_response {
            // If the response is successful, return an HTTP 200 OK response
            // with the slim_user data as JSON
            Ok(slim_user) => HttpResponse::Ok().json(slim_user),
            
            // If the response is an error, return an appropriate error response
            // using the error_response() method
            Err(err) => err.error_response(),
        }
    })
}