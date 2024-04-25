use axum::{
    debug_handler,
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Deserialize;
use sqlx::{Decode, Row};
use utoipa::IntoParams;
// use futures::TryStreamExt;

#[derive(Deserialize, IntoParams)]
pub struct CreateUser {
    name: String,
}

#[utoipa::path(
    post,
    path = "/api/user/create",
    params(CreateUser),
    responses(
        (status = 200, description = "User sucessfully created.", body = String),
        // (status = 401, description = "Invalid API Key"),
        // (status = 500, description = "Server error"),
    ),
    security(
        ("api_key" = [])
    ),
    tag = "User"
)]
pub async fn create_user(
    State(state): State<crate::AppState>,
    Query(payload): Query<CreateUser>,
) -> Response {
    let pool = state.pool.read().unwrap();
    // doesnt work for some reason
    // let rows = sqlx::query(r#"SELECT COUNT(*) > 0 FROM "user" WHERE name = $1"#)
    //     .bind(&payload.name)
    //     .fetch_one(&*pool)
    //     .await
    //     .unwrap();
    // let t: bool = rows.get(0);
    let t: bool = false;
    // println!("Is user `{}` nieuwe?: {}", payload.name, t);
    (StatusCode::OK, payload.name).into_response()
    // (StatusCode::OK, "a").into_response()
}
