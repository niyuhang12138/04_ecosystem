use anyhow::Result;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use http::{header::LOCATION, HeaderMap, StatusCode};
use nanoid::nanoid;
use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::{format::FmtSpan, Layer},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer as _,
};

const LISTENER_ADDR: &str = "localhost:8080";

#[derive(Debug, Deserialize)]
struct ShortenReq {
    url: String,
}

#[derive(Debug, Serialize)]
struct ShortenRes {
    url: String,
}

#[derive(Debug, Clone)]
struct AppState {
    db: MySqlPool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let layer = Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry().with(layer).init();

    let url = "mysql://root:root@localhost:3306/shortener";

    let state = AppState::try_new(url).await?;
    info!("Connected to database: {url}");

    let listener = TcpListener::bind(LISTENER_ADDR).await?;
    info!("Listening on: http://{}", LISTENER_ADDR);

    let app = Router::new()
        .route("/", post(shorten))
        .route("/{id}", get(redirect))
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}

async fn shorten(
    State(state): State<AppState>,
    Json(data): Json<ShortenReq>,
) -> Result<impl IntoResponse, StatusCode> {
    let id = state.create(&data.url).await.map_err(|e| {
        warn!("Failed to shorten URL: {e}");
        StatusCode::UNPROCESSABLE_ENTITY
    })?;
    let body = Json(ShortenRes {
        url: format!("http://{LISTENER_ADDR}/{id}"),
    });

    Ok((StatusCode::CREATED, body))
}

async fn redirect(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, StatusCode> {
    let url = state
        .get_url(&id)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mut headers = HeaderMap::new();
    headers.insert(LOCATION, url.parse().unwrap());
    Ok((StatusCode::FOUND, headers))
}

impl AppState {
    async fn try_new(url: &str) -> Result<Self> {
        let pool = MySqlPool::connect(url).await?;
        // Crate table if not exists
        sqlx::query(
            r#"
            CREATE TABLE IF NOT EXISTS urls (
                id CHAR(6) PRIMARY KEY,
                url TEXT NOT NULL
            )
            "#,
        )
        .execute(&pool)
        .await?;

        Ok(Self { db: pool })
    }

    async fn create(&self, url: &str) -> Result<String> {
        let id = nanoid!(6);

        sqlx::query("INSERT INTO urls(id, url) VALUES(?, ?);")
            .bind(&id)
            .bind(url)
            .execute(&self.db)
            .await?;

        info!("Inserted url: {url}");

        Ok(id)
    }

    async fn get_url(&self, id: &str) -> Result<String> {
        let record: (String,) = sqlx::query_as("SELECT url FROM urls WHERE id = ?")
            .bind(id)
            .fetch_one(&self.db)
            .await?;

        Ok(record.0)
    }
}
