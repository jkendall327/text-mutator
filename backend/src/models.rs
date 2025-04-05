#[derive(serde::Deserialize)]
pub struct MutationRequest {
    pub text: String,
    pub config: MutationOptions,
}

#[derive(serde::Deserialize)]
pub struct MutationOptions {
    pub mutation_rate: f32,
    pub allow_swaps: bool,
    pub allow_punctuation_removal: bool,
    pub allow_homophones: bool,
    pub seed: Option<u64>,
}

pub struct MutationResponse {
    pub mutated_text: String,
    pub mutations: Vec<MutationItemDto>,
}

pub struct MutationItem {
    start: usize,
    end: usize,
    r#type: Mutation,
}

#[derive(serde::Serialize)]
pub struct MutationResponseDto {
    pub mutatedText: String,
    pub mutations: Vec<MutationItemDto>,
}

#[derive(serde::Serialize)]
pub struct MutationItemDto {
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
