use artigo::models;
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use sqlx::{migrate, postgres::PgPoolOptions};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_PKG_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()?;

    let pool = get_database().await?;
    migrate!("./migrations").run(&pool).await?;

    let app = Router::new()
        .route("/status", get(|| async { "OK" }))
        .route("/articles", get(models::get_articles))
        .route("/articles", post(models::create_article))
        .route("/articles/{slug}", get(models::get_article_by_slug))
        .route("/articles/{slug}", put(models::update_article))
        .route("/articles/{slug}", delete(models::delete_article))
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await?;

    info!("Starting server on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

async fn get_database() -> anyhow::Result<sqlx::PgPool> {
    dotenvy::dotenv()?;
    let db_url = std::env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new()
        .max_connections(20)
        .connect(&db_url)
        .await?;
    info!("Established database connection");
    Ok(pool)
}
