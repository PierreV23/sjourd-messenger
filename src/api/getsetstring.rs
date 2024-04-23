use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

pub async fn getstring(State(state): State<crate::AppState>) -> Response {
    (StatusCode::OK, state.string.read().unwrap().clone()).into_response()
}

pub async fn setstring(
    State(state): State<crate::AppState>,
    Json(newstring): Json<String>,
) -> Response {
    let mut lock = state.string.write().unwrap();
    *lock = newstring;
    StatusCode::OK.into_response()
}
