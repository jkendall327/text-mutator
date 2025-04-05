use axum::{http::{StatusCode, Uri}, response::IntoResponse, Json};

use crate::{models::*, mutator::TextMutator};

pub async fn health() -> &'static str {
    "Healthy"
}

pub async fn fallback(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("Invalid request path: {uri}"))
}

#[axum::debug_handler]
pub async fn mutate(Json(payload): Json<MutationRequest>) -> impl IntoResponse {
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
        mutations: response.mutations.iter().map(|f| {
            let mapped_type = match f.r#type {
                Mutation::SwapLetters(_) => MutationDto::SwapLetters,
                Mutation::RemovePunctuation(_) => MutationDto::RemovePunctuation,
                Mutation::ReplaceHomophone(_, _) => MutationDto::ReplaceHomophone,
            };

            MutationItemDto {
                start: f.start,
                end: f.end,
                r#type: mapped_type
            }
        }).collect(),
    };

    Json(response)
}
