use anyhow::Context;
use axum::{
    extract::{ConnectInfo, Request},
    middleware::Next,
    response::Response,
};
use serde_json::json;
use std::{fmt::Display, net::SocketAddr};
use time::{format_description::well_known::Rfc3339, OffsetDateTime};
use tracing::info;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

pub fn init_tracing() -> anyhow::Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(EnvFilter::new("info"))
        .with(fmt::layer().json())
        .try_init()
        .context("initialize tracing subscriber")?;

    Ok(())
}

pub fn log_error(error: &impl Display) {
    let now = OffsetDateTime::now_utc().format(&Rfc3339).unwrap();
    let error = serde_json::to_string(&json!({
        "timestamp": now,
        "level": "ERROR",
        "message": "process exited with ERROR",
        "error": format!("{error:#}")
    }));

    // Not using `eprintln!`, because `tracing_subscriber::fmt` uses stdout by default.
    println!("{}", error.unwrap());
}

pub async fn requests_stats(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    req: Request,
    next: Next,
) -> Response {
    info!("{}:{}", addr.ip(), addr.port());
    next.run(req).await
}
