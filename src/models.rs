use crate::AppError;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::DateTime;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

#[derive(FromRow, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Article {
    pub id: i32,
    pub slug: String,
    pub title: String,
    pub body: String,
    pub hero_image: Option<String>,
    pub tags: Option<Vec<String>>,
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
    pub tags: Option<Vec<String>>,
}

pub async fn get_articles(
    State(pool): State<PgPool>,
) -> Result<(StatusCode, Json<Vec<Article>>), AppError> {
    let articles = sqlx::query_as("SELECT * FROM articles")
        .fetch_all(&pool)
        .await?;
    Ok((StatusCode::OK, Json(articles)))
}

pub async fn get_article_by_slug(
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<(StatusCode, Json<Article>), AppError> {
    let article = sqlx::query_as("SELECT * FROM articles WHERE slug = $1")
        .bind(slug)
        .fetch_one(&pool)
        .await?;
    Ok((StatusCode::OK, Json(article)))
}

pub async fn create_article(
    State(pool): State<PgPool>,
    Json(article): Json<ArticleRequest>,
) -> Result<(StatusCode, Json<Article>), AppError> {
    let article = sqlx::query_as("INSERT INTO articles (slug, title, body, hero_image, tags) VALUES ($1, $2, $3, $4, $5) RETURNING *")
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
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
    Json(article): Json<ArticleRequest>,
) -> Result<(StatusCode, Json<Article>), AppError> {
    let article = sqlx::query_as("UPDATE articles SET slug = $1, title = $2, body = $3, hero_image = $4, tags = $5 WHERE slug = $6 RETURNING *")
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
    State(pool): State<PgPool>,
    Path(slug): Path<String>,
) -> Result<StatusCode, AppError> {
    sqlx::query("DELETE FROM articles WHERE slug = $1")
        .bind(slug)
        .execute(&pool)
        .await?;
    Ok(StatusCode::NO_CONTENT)
}
