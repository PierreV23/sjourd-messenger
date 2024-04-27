use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use axum::{routing::post, Router};
use serde::{Deserialize, Serialize};
use sqlx::Row;
use utoipa::IntoParams;

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
        (status = 409, description = "User already exists."),
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
    let pool = state.pool;
    // doesnt work for some reason
    let rows = sqlx::query(r#"SELECT COUNT(*) != 1 FROM "user" WHERE name = $1"#)
        .bind(&payload.name)
        .fetch_one(&pool)
        .await
        .unwrap();
    let t: bool = rows.get(0);
    // let t: bool = false;
    println!("Is user `{}` nieuwe?: {}", payload.name, t);
    if t {
        let res = sqlx::query_as::<_, User>(r#"INSERT INTO "user" (name) VALUES ($1) RETURNING *"#)
            .bind(&payload.name)
            .fetch_one(&pool)
            .await
            .unwrap();

        (
            StatusCode::OK,
            format!("User Created {} {:?}", res.user_id, res.status),
        )
            .into_response()
    } else {
        (StatusCode::CONFLICT, "User Already Exists").into_response()
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status")]
pub enum Status {
    Online,
    Offline,
    Away,
    DoNotDisturb,
}

#[derive(sqlx::FromRow, Debug)]
#[allow(dead_code)]
pub struct User {
    user_id: i32,
    name: String,
    nickname: Option<String>,
    bio: Option<String>,
    status: Status,
}

pub fn router() -> Router<crate::AppState> {
    Router::new().route("/create", post(create_user))
}
