mod homophones;
mod models;
mod mutator;

use axum::{
    Json, Router,
    http::{HeaderValue, Method},
    response::IntoResponse,
    routing::{get, post},
};
use models::{MutationItemDto, MutationRequest, MutationResponseDto};
use mutator::TextMutator;
use std::io::{self};
use tower_http::cors::CorsLayer;
use tracing::{Level, info};
use tracing_subscriber::FmtSubscriber;

/// A program that deliberately introduces minor errors into text for proofreading practice

#[tokio::main]
async fn main() -> io::Result<()> {
    // Initialize tracing subscriber
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set tracing subscriber");

    info!("Starting text-mutator");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::POST, Method::GET]);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/mutate", post(mutate))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn health() -> &'static str {
    "Healthy"
}

#[axum::debug_handler]
async fn mutate(Json(payload): Json<MutationRequest>) -> impl IntoResponse {
    // Set mutation flags
    let swap_letters = payload.config.allow_swaps;
    let remove_punctuation = payload.config.allow_punctuation_removal;
    let homophones = payload.config.allow_homophones;

    // Apply mutations
    let mut text_mutator = TextMutator::new(
        payload.config.mutation_rate,
        payload.config.seed,
        swap_letters,
        remove_punctuation,
        homophones,
    );

    let response = text_mutator.mutate(&payload.text);

    let response = MutationResponseDto {
        mutated_text: response.mutated_text,
        mutations: response.mutations.iter().map(|f| MutationItemDto {
            start: 1,
            end: 2,
            r#type: models::MutationDto::SwapLetters
        }).collect(),
    };

    Json(response)
}
