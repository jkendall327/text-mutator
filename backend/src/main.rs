mod handler;
mod homophones;
mod models;
mod mutator;

use anyhow::bail;
use axum::{
    Router,
    http::{HeaderValue, Method},
    routing::{get, post},
};
use handler::*;
use std::borrow::Cow;
use tower_http::cors::CorsLayer;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

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
        .route("/api/health", get(health))
        .route("/api/mutate", post(mutate))
        .fallback(fallback)
        .layer(cors);

    let backend_url = env.backend_url.to_string();

    let listener = tokio::net::TcpListener::bind(backend_url).await.unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Clone, Debug)]
pub struct EnvironmentVariables {
    pub frontend_url: Cow<'static, str>,
    pub backend_url: Cow<'static, str>,
}

impl EnvironmentVariables {
    pub fn from_env() -> anyhow::Result<Self> {
        dotenv::dotenv().ok();

        Ok(Self {
            frontend_url: match dotenv::var("MUTATOR_FRONTEND_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing frontend URL: {err}"),
            },
            backend_url: match dotenv::var("MUTATOR_BACKEND_URL") {
                Ok(url) => url.into(),
                Err(err) => bail!("missing backend URL: {err}"),
            },
        })
    }
}
