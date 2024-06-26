use axum::{
    routing::{get, post},
    Router,
};

pub mod getsetstring;
pub mod ping;
pub mod user;

pub fn router() -> Router<crate::AppState> {
    Router::new()
        .route("/ping", get(ping::ping))
        .route("/setstring", post(getsetstring::setstring))
        .route("/getstring", get(getsetstring::getstring))
        .nest("/user", user::router())
}
