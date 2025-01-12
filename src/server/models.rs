use super::AppError;
use axum::{extract::State, http::StatusCode, Json};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool};

#[derive(FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub hero_image: Option<String>,
    pub tags: Option<String>, // make vec in pg
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<chrono::Utc>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub updated_at: DateTime<chrono::Utc>,
}

#[axum::debug_handler]
pub async fn get_articles(
    State(pool): State<SqlitePool>,
) -> Result<(StatusCode, Json<Vec<Article>>), AppError> {
    let articles = sqlx::query_as("SELECT * FROM articles")
        .fetch_all(&pool)
        .await?;
    Ok((StatusCode::OK, Json(articles)))
}
