use sqlx::postgres::PgPoolOptions;
use std::sync::{Arc, RwLock};

use axum::{routing::get, Router};
pub mod api;

#[derive(Clone)]
pub struct AppState {
    pub string: Arc<RwLock<String>>,
    // pub pool: Arc<RwLock<>
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let p = PgPoolOptions::new()
        .max_connections(5)
        .connect(
            std::env::var("DB_URL")
                .expect("DB_URL must be set")
                .as_str(),
        )
        .await
        .unwrap();

    let state = AppState {
        string: Arc::new(RwLock::new(String::from("hey"))),
    };

    let app = Router::<AppState>::new()
        .nest("/api", api::router())
        .route("/", get(|| async { "Hello, World!" }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
