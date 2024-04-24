use sqlx::{Pool, Postgres};

pub async fn reset(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query("DROP SCHEMA public CASCADE;")
        .execute(pool)
        .await?;
    sqlx::query("CREATE SCHEMA public;").execute(pool).await?;
    Ok(())
}

pub async fn create_tables(pool: &Pool<Postgres>) -> Result<(), sqlx::Error> {
    sqlx::query("CREATE TYPE user_status AS ENUM ('Online', 'Offline', 'Away', 'DoNotDisturb');")
        .execute(pool)
        .await?;
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS \"user\" (
            id INTEGER PRIMARY KEY,
            name TEXT UNIQUE NOT NULL,
            nickname TEXT,
            bio TEXT,
            status user_status NOT NULL
        )",
    )
    .execute(pool)
    .await?;
    Ok(())
}
