use std::time::Duration;

use axum::{routing::get, Router};
use tokio::{
    net::TcpListener,
    time::{sleep, Instant},
};
use tracing::{debug, info, instrument, level_filters::LevelFilter, warn};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    Layer,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let console = fmt::Layer::new()
        .with_span_events(FmtSpan::CLOSE)
        .pretty()
        .with_filter(LevelFilter::ERROR);

    let file_appender = tracing_appender::rolling::hourly("tmp/logs", "ecosystem.log");

    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let file = fmt::Layer::new()
        .with_writer(non_blocking)
        .pretty()
        .with_filter(LevelFilter::INFO);

    tracing_subscriber::registry()
        .with(console)
        .with(file)
        .init();

    let addr = "0.0.0.0:8080";
    info!("Listener on {addr}");
    let app = Router::new().route("/", get(index_handler));
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

#[instrument]
async fn index_handler() -> &'static str {
    debug!("index handler started");
    sleep(Duration::from_millis(10)).await;
    let ret = long_test().await;
    info!(http.status = 200, "index handler completed");
    ret
}

#[instrument]
async fn long_test() -> &'static str {
    let start = Instant::now();
    let dur = 112;
    sleep(Duration::from_millis(dur)).await;
    let elapsed = start.elapsed().as_millis() as u64;
    warn!(app.test_duration = elapsed, "task takes too long");
    "Hello World!"
}
