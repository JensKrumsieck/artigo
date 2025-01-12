pub mod models;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use sqlx::sqlite::SqlitePoolOptions;
use std::{fs, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn serve() -> anyhow::Result<()> {
    let pool = get_database().await?;

    let app = Router::new()
        .route("/status", get(|| async { "OK" }))
        .route("/articles", get(models::get_articles))
        .route("/articles", post(models::create_article))
        .route("/articles/{slug}", get(models::get_article_by_slug))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    axum::serve(listener, app).await?;
    Ok(())
}

pub async fn migrate() -> anyhow::Result<()> {
    let db_path = "./artigo.db";

    if !fs::exists(db_path)? {
        fs::File::create(db_path)?;
    }

    let pool = get_database().await?;
    sqlx::migrate!("./migrations").run(&pool).await?;

    Ok(())
}

async fn get_database() -> anyhow::Result<sqlx::SqlitePool> {
    dotenvy::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;
    Ok(pool)
}

pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
