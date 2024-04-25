use axum::{routing::post, Router};

pub mod create;

pub fn router() -> Router<crate::AppState> {
    Router::new().route("/create", post(create::create_user))
}
