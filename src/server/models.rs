use super::AppError;
use axum::{extract::{Path, State}, http::StatusCode, Json};
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

#[derive(FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArticleRequest {
    pub slug: String,
    pub title: String,
    pub body: String,
    pub hero_image: Option<String>,
    pub tags: Option<String>,
}

pub async fn get_articles(
    State(pool): State<SqlitePool>,
) -> Result<(StatusCode, Json<Vec<Article>>), AppError> {
    let articles = sqlx::query_as("SELECT * FROM articles")
        .fetch_all(&pool)
        .await?;
    Ok((StatusCode::OK, Json(articles)))
}

pub async fn get_article_by_slug(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
) -> Result<(StatusCode, Json<Article>), AppError> {
    let article = sqlx::query_as("SELECT * FROM articles WHERE slug = ?")
        .bind(slug)
        .fetch_one(&pool)
        .await?;
    Ok((StatusCode::OK, Json(article)))
}

pub async fn create_article(
    State(pool): State<SqlitePool>,
    Json(article): Json<ArticleRequest>,
) -> Result<(StatusCode, Json<Article>), AppError> {
    let article = sqlx::query_as("INSERT INTO articles (slug, title, body, hero_image, tags) VALUES (?, ?, ?, ?, ?) RETURNING *")
        .bind(article.slug)
        .bind(article.title)
        .bind(article.body)
        .bind(article.hero_image)
        .bind(article.tags)
        .fetch_one(&pool)
        .await?;
    Ok((StatusCode::CREATED, Json(article)))
}

pub async fn update_article(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
    Json(article): Json<ArticleRequest>,
) -> Result<(StatusCode, Json<Article>), AppError> {
    let article = sqlx::query_as("UPDATE articles SET slug = ?, title = ?, body = ?, hero_image = ?, tags = ? WHERE slug = ? RETURNING *")
        .bind(article.slug)
        .bind(article.title)
        .bind(article.body)
        .bind(article.hero_image)
        .bind(article.tags)
        .bind(slug)
        .fetch_one(&pool)
        .await?;
    Ok((StatusCode::OK, Json(article)))
}

pub async fn delete_article(
    State(pool): State<SqlitePool>,
    Path(slug): Path<String>,
) -> Result<StatusCode, AppError> {
    sqlx::query("DELETE FROM articles WHERE slug = ?")
        .bind(slug)
        .execute(&pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}