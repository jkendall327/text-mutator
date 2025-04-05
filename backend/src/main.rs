mod homophones;
mod mutator;

use axum::{
    http::{HeaderValue, Method}, response::{Html, IntoResponse}, routing::{get, post}, Json, Router
};
use mutator::TextMutator;
use std::io::{self, Write};
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

    return Ok(());

    // Set mutation flags
    let swap_letters = true;
    let remove_punctuation = true;
    let homophones = true;

    // If no specific mutations are enabled, default to all
    let (swap_letters, remove_punctuation, homophones) =
        if !swap_letters && !remove_punctuation && !homophones {
            (true, true, true)
        } else {
            (swap_letters, remove_punctuation, homophones)
        };

    // Read input
    let mut input = String::new();
    println!("Please enter input:");
    io::stdin().read_line(&mut input)?;

    // Apply mutations
    let mut text_mutator =
        TextMutator::new(1.0, None, swap_letters, remove_punctuation, homophones);

    let (mutated_text, num_mutations) = text_mutator.mutate(&input);

    // Write output
    io::stdout().write_all(mutated_text.as_bytes())?;
    eprintln!("\n--- Added {} mutations to the text ---", num_mutations);

    Ok(())
}

async fn health() -> Html<&'static str> {
    Html("Healthy")
}

async fn mutate() -> impl IntoResponse {
    Json(vec!["Hello", "World"])
}
