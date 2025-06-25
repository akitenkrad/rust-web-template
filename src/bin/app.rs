use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};
use tokio::net::TcpListener;

use adapter::database::connect_database_with;
use anyhow::{Context as _, Result};
use axum::{Router, http::Method};
use clap::{Parser, Subcommand};
use registry::AppRegistryImpl;
use shared::{config::AppConfig, logger::init_logger, utils::generate_progress_bar};
use tower_http::{
    cors::{self, CorsLayer},
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use tracing::Level;
use web_app::route::v1;

#[derive(Debug, Parser, Clone)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    subcommand: SubCommands,
}

#[derive(Subcommand, Debug, Clone)]
enum SubCommands {
    /// Start the web application
    StartServer(StartServerArgs),
}

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct StartServerArgs {}

fn cors() -> CorsLayer {
    CorsLayer::new()
        .allow_headers(cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_origin(cors::Any)
}

async fn bootstrap() -> Result<()> {
    let app_config = AppConfig::new()?;
    let pool = connect_database_with(&app_config.database);
    let registry = Arc::new(AppRegistryImpl::new(pool));

    let app = Router::new()
        .merge(v1::routes())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .latency_unit(tower_http::LatencyUnit::Millis),
                ),
        )
        .layer(cors())
        .with_state(registry);

    let addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080);
    let listener = TcpListener::bind(addr).await?;
    tracing::info!("Listening on {}", addr);
    axum::serve(listener, app)
        .await
        .context("Unexpected error happened in server")
        .inspect_err(|e| {
            tracing::error!(
                error.cause_chain = ?e,error.message = %e, "Unexpected error"
            )
        })
}

async fn start_server(_args: &StartServerArgs) {
    // Start the web application
    tracing::info!("Starting web application...");
    bootstrap().await.expect("Failed to start web application");
}

#[tokio::main]
async fn main() {
    init_logger().expect("Failed to initialize logger");
    let cli = Cli::parse();

    match &cli.subcommand {
        SubCommands::StartServer(args) => start_server(args).await,
    }
}
