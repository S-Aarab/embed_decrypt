use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    panic,
    time::Duration,
};
use tokio::{
    net::TcpListener,
    signal::unix::{signal, SignalKind},
    time::sleep,
};
use tower::ServiceBuilder;
use tower_http::trace::{self, TraceLayer};
use tracing::{error, info, Level};

use anyhow::Context;
use axum::{
    extract::Query, http::StatusCode, middleware::from_fn_with_state, response::Json, routing::get,
    Router,
};
mod logging;
use logging::{init_tracing, log_error, requests_stats};
mod embed;
use embed::{handle_embed, EmbedSources};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Embed {
    embed_url: String,
    referrer: String,
}

#[tokio::main]
async fn main() {
    let result = init_tracing();
    if let Err(ref error) = result {
        log_error(error);
    }

    panic::set_hook(Box::new(|panic| error!(%panic, "process panicked")));

    if let Err(ref error) = run().await {
        error!(
            error = format!("{error:#}"),
            backtrace = %error.backtrace(),
            "process exited with ERROR"
        );
    }
}

async fn home() -> (StatusCode, &'static str) {
    (StatusCode::OK, "Welcome to the home page!")
}

async fn parse_embed_id(route_params: Query<Embed>) -> Json<EmbedSources> {
    let embed_sources = handle_embed(&route_params.embed_url, &route_params.referrer);

    Json(embed_sources)
}

async fn shutdown_signal(shutdown_timeout: Option<Duration>) {
    signal(SignalKind::terminate())
        .expect("install SIGTERM handler")
        .recv()
        .await;

    if let Some(shutdown_timeout) = shutdown_timeout {
        sleep(shutdown_timeout).await;
    }
}

async fn run() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    let port = std::env::var("PORT")
        .unwrap_or("3000".into())
        .parse::<u16>()?;

    info!("Server started on http://localhost:{:#?}", port);

    let app = Router::new()
        .route("/", get(home))
        .route("/embed", get(parse_embed_id))
        .route_layer(from_fn_with_state("state", requests_stats))
        .layer(
            ServiceBuilder::new().layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                    .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
            ),
        );

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), port);

    let listener = TcpListener::bind(&addr).await.context("bind TcpListener")?;

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .with_graceful_shutdown(shutdown_signal(None))
    .await
    .context("run server")
}
