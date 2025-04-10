use axum::{
    Json,
    http::{StatusCode, Uri},
    response::IntoResponse,
};
use tracing::info;

use crate::{
    models::{
        Mutation, MutationRequest, MutationResponse, MutationResponseItem, MutationResponseType,
    },
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

    let response = MutationResponse {
        mutated_text: response.mutated_text,
        mutations: response
            .mutations
            .iter()
            .map(|f| match f {
                Mutation::SwapLetters(i) => MutationResponseItem {
                    start: *i,
                    end: *i,
                    r#type: MutationResponseType::SwapLetters,
                },
                Mutation::RemovePunctuation(i) => MutationResponseItem {
                    start: *i,
                    end: *i,
                    r#type: MutationResponseType::RemovePunctuation,
                },
                Mutation::ReplaceHomophone(i, e) => MutationResponseItem {
                    start: *i,
                    end: *e,
                    r#type: MutationResponseType::ReplaceHomophone,
                },
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
