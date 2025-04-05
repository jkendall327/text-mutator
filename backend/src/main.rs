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
use tower_http::cors::CorsLayer;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

const CURRENT_VERSION: usize = 1;

/// A program that deliberately introduces minor errors into text for proofreading practice
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    info!("Starting text-mutator");

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

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

fn get_route<S: AsRef<str>>(endpoint: S) -> String {
    let str = endpoint.as_ref();
    format!("/api/v{CURRENT_VERSION}/{str}")
}
