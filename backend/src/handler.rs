use axum::{
    Json,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use tracing::info;

use crate::{
    models::{Mutation, MutationDto, MutationItemDto, MutationRequest, MutationResponseDto},
    mutator::TextMutator,
};

pub async fn health() -> &'static str {
    "Healthy"
}

pub async fn fallback(uri: Uri) -> (StatusCode, String) {
    (
        StatusCode::NOT_FOUND,
        format!("Invalid request path: {uri}"),
    )
}

/// Arbitrary amount, chosen just to prevent degenerate requests.
pub const MAX_INPUT_LENGTH: usize = 5000;

#[axum::debug_handler]
pub async fn mutate(Json(payload): Json<MutationRequest>) -> impl IntoResponse {
    let length = payload.text.chars().count();

    if length > MAX_INPUT_LENGTH {
        let error = format!(
            "The input text was over the max length of {MAX_INPUT_LENGTH} characters ({length})"
        );

        return (StatusCode::BAD_REQUEST, error).into_response();
    }

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
        mutations: response
            .mutations
            .iter()
            .map(|f| {
                let mapped_type = match f.r#type {
                    Mutation::SwapLetters(_) => MutationDto::SwapLetters,
                    Mutation::RemovePunctuation(_) => MutationDto::RemovePunctuation,
                    Mutation::ReplaceHomophone(_, _) => MutationDto::ReplaceHomophone,
                };

                MutationItemDto {
                    start: f.start,
                    end: f.end,
                    r#type: mapped_type,
                }
            })
            .collect(),
    };

    let debug_response = serde_json::to_string(&response);

    if debug_response.is_ok() {
        info!("Sending response: {:?}", debug_response);
    } else {
        info!("Serializing the response failed!");
    }

    Json(response).into_response()
}
