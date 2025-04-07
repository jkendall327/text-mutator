// Contracts

/// Represents a request to mutate a passage of text.
#[derive(serde::Serialize, serde::Deserialize)]
pub struct MutationRequest {
    /// The passage to mutate.
    pub text: String,

    /// Optional settings determining the nature of the mutation.
    pub config: MutationOptions,
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct MutationOptions {
    #[serde(rename = "mutationRate")]
    pub mutation_rate: f32,

    #[serde(rename = "allowSwaps")]
    pub allow_swaps: bool,

    #[serde(rename = "allowPunctuationRemoval")]
    pub allow_punctuation_removal: bool,

    #[serde(rename = "allowHomophones")]
    pub allow_homophones: bool,

    #[serde(rename = "seed")]
    pub seed: Option<u64>,
}

/// Represents the result of a mutation applied to a passage of text.
#[derive(serde::Serialize)]
pub struct MutationResponseDto {
    /// The mutated passage.
    pub mutated_text: String,

    /// A collection indicating what mutations were applied, and where.
    pub mutations: Vec<MutationItemDto>,
}

#[derive(serde::Serialize)]
pub struct MutationItemDto {
    /// The character-based index where, in the mutated passage, this mutation begins.
    pub start: usize,

    /// The character-based index where, in the mutated passage, this mutation ends.
    pub end: usize,

    /// The type of mutation indicated by this item.
    pub r#type: MutationDto,
}

/// A mutation that can be applied to text
#[derive(serde::Serialize)]
pub enum MutationDto {
    /// Swaps a letter with the next letter.
    SwapLetters,

    /// Removes punctuation.
    RemovePunctuation,

    /// Replaces a word with a homophone.
    ReplaceHomophone,
}

// Domain types

pub(crate) struct MutationItem {
    pub start: usize,
    pub end: usize,
    pub r#type: Mutation,
}

pub(crate) enum Mutation {
    SwapLetters(usize),             // Swap with next letter
    RemovePunctuation(usize),       // Remove punctuation at index
    ReplaceHomophone(usize, usize), // Replace word at index with length
}

pub(crate) struct MutationResponse {
    pub mutated_text: String,
    pub mutations: Vec<MutationItem>,
}
