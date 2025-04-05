pub struct MutationRequest {
    text: String,
    config: MutationOptions,
}

pub struct MutationOptions {
    mutation_rate: f32,
    allow_swaps: bool,
    allow_punctuation_removal: bool,
    allow_homophones: bool,
}

#[derive(serde::Serialize)]
pub struct MutationResponse {
    pub mutatedText: String,
    pub mutations: Vec<MutationItem>,
}

#[derive(serde::Serialize)]
pub struct MutationItem {
    start: usize,
    end: usize,
    r#type: Mutation,
}

/// A mutation that can be applied to text
#[derive(serde::Serialize)]
pub enum Mutation {
    SwapLetters(usize),             // Swap with next letter
    RemovePunctuation(usize),       // Remove punctuation at index
    ReplaceHomophone(usize, usize), // Replace word at index with length
}