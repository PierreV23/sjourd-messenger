use docs::ApiDoc;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::{Arc, RwLock};
use utoipa::OpenApi;
use utoipa_rapidoc::RapiDoc;

use axum::{routing::get, Json, Router};
pub mod api;
pub mod database;
pub mod docs;

#[derive(Clone)]
pub struct AppState {
    pub string: Arc<RwLock<String>>,
    pub pool: Arc<RwLock<Pool<Postgres>>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "user_status")]
pub enum Status {
    Online,
    Offline,
    Away,
    DoNotDisturb,
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            std::env::var("DB_URL")
                .expect("DB_URL must be set")
                .as_str(),
        )
        .await
        .unwrap();

    database::reset(&pool)
        .await
        .expect("Resetting database failed.");
    database::create_tables(&pool)
        .await
        .expect("Creating tables failed.");

    let state = AppState {
        string: Arc::new(RwLock::new(String::from("hey"))),
        pool: Arc::new(RwLock::new(pool)),
    };
    let app = Router::<AppState>::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/api", api::router())
        // .route("/api-docs/openapi.json", get(crate::docs::openapi))
        // .route("/api-docs/openapi.json", get( || async { Json(crate::docs::ApiDoc::openapi()) }))
        // .merge(RapiDoc::new("/api-docs/openapi.json").path("/docs"))
        .merge(RapiDoc::with_openapi("/api-docs/openapi.json", ApiDoc::openapi()).path("/docs"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
