use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[utoipa::path(
    get,
    path = "/api/ping",
    responses(
        (status = 200, description = "Pong", body = String),
        // (status = 401, description = "Invalid API Key"),
        // (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "Misc"
)]
pub async fn ping() -> Response {
    (StatusCode::OK, "pong!").into_response()
}
