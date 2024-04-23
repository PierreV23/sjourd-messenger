use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub async fn ping() -> Response {
    (StatusCode::OK, "pong!").into_response()
}
