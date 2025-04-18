mod env;
mod handler;
mod homophones;
mod models;
mod mutator;

use std::time::Duration;

use axum::{
    Router,
    body::Body,
    http::{HeaderValue, Method, Request},
    response::Response,
    routing::{get, post},
};
use env::EnvironmentVariables;
use tokio::signal;
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};
use tracing::{Level, Span};
use tracing_appender::rolling;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;

const CURRENT_VERSION: usize = 1;

/// A program that deliberately introduces minor errors into text for proofreading practice
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let env = EnvironmentVariables::from_env()?;

    setup_logging();

    let app = app(&env);

    let backend_url = env.backend_url.to_string();

    let listener = tokio::net::TcpListener::bind(backend_url.clone())
        .await
        .unwrap();

    tracing::info!("listening on {}", backend_url.clone());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();

    Ok(())
}

fn setup_logging() {
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
}

fn app(env: &EnvironmentVariables) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(env.frontend_url.parse::<HeaderValue>().unwrap())
        .allow_headers(Any)
        .allow_methods([Method::POST, Method::GET]);

    let tracer = TraceLayer::new_for_http()
        .make_span_with(|request: &Request<Body>| {
            let path = request.uri().path().to_owned();
            let method = request.method().clone();

            tracing::span!(
                Level::INFO,
                "request",
                method = %method,
                path = %path,
                request_id = %Uuid::new_v4(),
            )
        })
        .on_response(|response: &Response, latency: Duration, _span: &Span| {
            tracing::info!(
                status = %response.status().as_u16(),
                latency = %latency.as_millis(),
                "finished processing request"
            );
        });

    let app = Router::new()
        .route(get_route("health").as_str(), get(handler::health))
        .route(get_route("mutate").as_str(), post(handler::mutate))
        .fallback(handler::fallback)
        .layer(cors)
        .layer(tracer);

    app
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

#[cfg(test)]
mod tests {
    use crate::{
        app,
        env::EnvironmentVariables,
        get_route, handler,
        models::{MutationRequest, MutationRequestOptions},
    };
    use axum::{
        Router,
        body::Body,
        http::{self, Request, Response, StatusCode},
    };
    use http_body_util::BodyExt;
    use serde_json::json;
    use tower::ServiceExt;

    #[tokio::test]
    async fn app_starts_up_and_serves_healthcheck() {
        let app = app(&EnvironmentVariables::empty());

        let response = app
            .oneshot(
                Request::builder()
                    .uri(get_route("health"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.into_body().collect().await.unwrap().to_bytes();
        assert_eq!(&body[..], b"Healthy");
    }

    #[tokio::test]
    async fn fake_endpoint_returns_404() {
        let app = app(&EnvironmentVariables::empty());

        let response = app
            .oneshot(
                Request::builder()
                    .uri(get_route("foobar"))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn mutate_returns_error_when_input_is_too_large() {
        let app = app(&EnvironmentVariables::empty());

        let too_big = handler::MAX_INPUT_LENGTH + 1;

        let invalid_input = "a".repeat(too_big);

        let req = MutationRequest {
            text: invalid_input,
            config: MutationRequestOptions::default(),
        };

        let response = send_json_request(app, req).await;

        assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    }

    async fn send_json_request(app: Router, req: MutationRequest) -> Response<Body> {
        app.oneshot(
            Request::builder()
                .method(http::Method::POST)
                .header(http::header::CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
                .uri(get_route("mutate"))
                .body(Body::from(serde_json::to_vec(&json!(req)).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap()
    }
}
