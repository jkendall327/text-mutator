mod env;
mod handler;
mod homophones;
mod models;
mod mutator;

use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{get, post},
};
use env::EnvironmentVariables;
use tokio::signal;
use tower_http::cors::CorsLayer;
use tracing_appender::rolling;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CURRENT_VERSION: usize = 1;

/// A program that deliberately introduces minor errors into text for proofreading practice
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let file_appender = rolling::daily("logs", "application.log");
    let (non_blocking_appender, _guard) = tracing_appender::non_blocking(file_appender);

    // Create the file layer with the rolling log configuration
    let file_layer = tracing_subscriber::fmt::layer()
        .with_writer(non_blocking_appender)
        .with_ansi(false); // Disable ANSI color codes in file output

    // Initialize tracing subscriber
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("{}=debug", env!("CARGO_CRATE_NAME")).into()),
        )
        .with(tracing_subscriber::fmt::layer().with_writer(std::io::stdout))
        .with(file_layer)
        .init();

    tracing::info!("Starting text-mutator");

    let env = EnvironmentVariables::from_env()?;

    let cors = CorsLayer::new()
        .allow_origin(env.frontend_url.parse::<HeaderValue>().unwrap())
        .allow_methods([Method::POST, Method::GET]);

    let app = Router::new()
        .route(get_route("health").as_str(), get(handler::health))
        .route(get_route("mutate").as_str(), post(handler::mutate))
        .fallback(handler::fallback)
        .layer(cors);

    let backend_url = env.backend_url.to_string();

    let listener = tokio::net::TcpListener::bind(backend_url).await.unwrap();

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

fn get_route<S: AsRef<str>>(endpoint: S) -> String {
    let str = endpoint.as_ref();
    format!("/api/v{CURRENT_VERSION}/{str}")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {
            tracing::info!("App shutting down...");
        },
        () = terminate => {},
    }
}
