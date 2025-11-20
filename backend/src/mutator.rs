use rand::rngs::StdRng;
use rand::{SeedableRng, seq::SliceRandom};
use std::sync::Arc;
use tracing::{debug, info, trace};

use crate::homophones::HomophoneSets;
use crate::models::{Mutation, MutationResult};

/// Applies mutations to text
pub struct TextMutator {
    mutation_rate: f32,
    rng: StdRng,
    swap_letters: bool,
    remove_punctuation: bool,
    use_homophones: bool,
    homophones: Arc<HomophoneSets>,
}

impl TextMutator {
    pub(crate) fn new(
        mutation_rate: f32,
        seed: Option<u64>,
        swap_letters: bool,
        remove_punctuation: bool,
        homophones: bool,
        homophone_sets: Arc<HomophoneSets>,
    ) -> Self {
        info!("Creating TextMutator with mutation_rate={}", mutation_rate);
        debug!(
            "Mutation flags: swap_letters={}, remove_punctuation={}, homophones={}",
            swap_letters, remove_punctuation, homophones
        );

        let rng = if let Some(seed_val) = seed {
            debug!("Using provided seed: {}", seed_val);
            StdRng::seed_from_u64(seed_val)
        } else {
            debug!("Using entropy-based seed");
            StdRng::from_os_rng()
        };

        TextMutator {
            mutation_rate,
            rng,
            swap_letters,
            remove_punctuation,
            use_homophones: homophones,
            homophones: homophone_sets,
        }
    }

    fn find_possible_mutations(&self, text: &str) -> Vec<Mutation> {
        trace!(
            "Finding possible mutations in text of length {}",
            text.len()
        );
        let mut mutations = Vec::new();
        let chars: Vec<char> = text.chars().collect();

        // Find possible letter swaps
        if self.swap_letters {
            trace!("Looking for possible letter swaps");
            for i in 0..chars.len().saturating_sub(1) {
                if chars[i].is_alphabetic() && chars[i + 1].is_alphabetic() {
                    mutations.push(Mutation::SwapLetters(i));
                }
            }
        }

        // Find punctuation that could be removed
        if self.remove_punctuation {
            trace!("Looking for punctuation to remove");
            for (i, c) in chars.iter().enumerate() {
                if c.is_ascii_punctuation() {
                    mutations.push(Mutation::RemovePunctuation(i));
                }
            }
        }

        // Find homophones that could be replaced
        if self.use_homophones {
            trace!("Looking for homophones to replace");
            let words: Vec<&str> = text.split_whitespace().collect();
            let mut char_index = 0;

            for word in words {
                // Skip past whitespace to get to the word
                while char_index < text.len() && !text[char_index..].starts_with(word) {
                    char_index += 1;
                }

                // Strip punctuation from word for homophone lookup
                let clean_word: String = word
                    .chars()
                    .filter(|c| c.is_alphabetic() || c == &'\'')
                    .collect();

                if !clean_word.is_empty()
                    && self.homophones.find_matching_set(&clean_word).is_some()
                {
                    trace!("Found homophone candidate: '{}'", clean_word);
                    mutations.push(Mutation::ReplaceHomophone(char_index, word.len()));
                }

                char_index += word.len();
            }
        }

        debug!("Found {} possible mutations", mutations.len());
        mutations
    }

    pub(crate) fn mutate(&mut self, text: &str) -> MutationResult {
        info!("Mutating text of length {}", text.len());
        let possible_mutations = self.find_possible_mutations(text);

        debug_assert!(self.mutation_rate >= 0.0);

        #[allow(clippy::cast_possible_truncation)]
        #[allow(clippy::cast_sign_loss)]
        #[allow(clippy::cast_precision_loss)]
        let num_mutations = (possible_mutations.len() as f32 * self.mutation_rate).floor() as usize;

        debug!(
            "Planning to apply {} mutations out of {} possible",
            num_mutations,
            possible_mutations.len()
        );

        if possible_mutations.is_empty() || num_mutations == 0 {
            info!("No mutations to apply");
            return MutationResult {
                mutated_text: text.to_string(),
                mutations: vec![],
            };
        }

        // Select which mutations to apply
        let mut selected_mutations = possible_mutations;
        selected_mutations.shuffle(&mut self.rng);
        selected_mutations.truncate(num_mutations);

        // Sort by position to apply from end to beginning (to avoid index shifts)
        selected_mutations.sort_by(|a, b| {
            let pos_a = match a {
                Mutation::SwapLetters(i)
                | Mutation::RemovePunctuation(i)
                | Mutation::ReplaceHomophone(i, _) => *i,
            };

            let pos_b = match b {
                Mutation::SwapLetters(i)
                | Mutation::RemovePunctuation(i)
                | Mutation::ReplaceHomophone(i, _) => *i,
            };

            pos_b.cmp(&pos_a) // Reverse order
        });

        let result = self.apply_mutations(text, &selected_mutations);

        MutationResult {
            mutated_text: result,
            mutations: selected_mutations,
        }
    }

    fn apply_mutations(&mut self, text: &str, selected_mutations: &Vec<Mutation>) -> String {
        let mut result = text.to_string();
        let mut actual_mutations = 0;
        debug!("Applying mutations from end to beginning to avoid index shifts");

        for mutation in selected_mutations {
            match mutation {
                Mutation::SwapLetters(i) => {
                    let i = *i;

                    let mut chars: Vec<char> = result.chars().collect();
                    if i + 1 < chars.len() {
                        trace!(
                            "Swapping letters at positions {} and {}: '{}' and '{}'",
                            i,
                            i + 1,
                            chars[i],
                            chars[i + 1]
                        );
                        chars.swap(i, i + 1);
                        result = chars.into_iter().collect();
                        actual_mutations += 1;
                    }
                }
                Mutation::RemovePunctuation(i) => {
                    let i = *i;

                    let mut chars: Vec<char> = result.chars().collect();
                    if i < chars.len() && chars[i].is_ascii_punctuation() {
                        trace!("Removing punctuation '{}' at position {}", chars[i], i);
                        chars.remove(i);
                        result = chars.into_iter().collect();
                        actual_mutations += 1;
                    }
                }
                Mutation::ReplaceHomophone(i, len) => {
                    let i = *i;
                    if i + len <= result.len() {
                        let word = &result[i..i + len];
                        let clean_word: String = word
                            .chars()
                            .filter(|c| c.is_alphabetic() || c == &'\'')
                            .collect();

                        if let Some(alternative) =
                            self.homophones.get_alternative(&clean_word, &mut self.rng)
                        {
                            trace!(
                                "Replacing homophone '{}' with '{}'",
                                clean_word, alternative
                            );

                            // Preserve trailing punctuation if any
                            let trailing_punct: String =
                                word.chars().filter(char::is_ascii_punctuation).collect();

                            let replacement = alternative + &trailing_punct;
                            result = result[..i].to_string() + &replacement + &result[i + len..];
                            actual_mutations += 1;
                        }
                    }
                }
            }
        }

        info!("Applied {} mutations", actual_mutations);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Mutation; // Use the internal Mutation enum

    // Helper to create a mutator with specific options and a fixed seed
    fn create_test_mutator(
        mutation_rate: f32,
        swap_letters: bool,
        remove_punctuation: bool,
        homophones: bool,
    ) -> TextMutator {
        // Use a fixed seed for deterministic tests
        TextMutator::new(
            mutation_rate,
            Some(42),
            swap_letters,
            remove_punctuation,
            homophones,
            Arc::new(HomophoneSets::new_for_tests()),
        )
    }

    #[test]
    fn test_find_possible_mutations_swap_only() {
        let mutator = create_test_mutator(1.0, true, false, false);
        let text = "abc";
        let mutations = mutator.find_possible_mutations(text);
        assert_eq!(mutations.len(), 2); // ab, bc
        assert!(matches!(mutations[0], Mutation::SwapLetters(0)));
        assert!(matches!(mutations[1], Mutation::SwapLetters(1)));
    }

    #[test]
    fn test_find_possible_mutations_punctuation_only() {
        let mutator = create_test_mutator(1.0, false, true, false);
        let text = "a,b.c!";
        let mutations = mutator.find_possible_mutations(text);
        assert_eq!(mutations.len(), 3); // , . !
        assert!(matches!(mutations[0], Mutation::RemovePunctuation(1)));
        assert!(matches!(mutations[1], Mutation::RemovePunctuation(3)));
        assert!(matches!(mutations[2], Mutation::RemovePunctuation(5)));
    }

    #[test]
    fn test_find_possible_mutations_homophones_only() {
        let mutator = create_test_mutator(1.0, false, false, true);
        let text = "your text"; // "your" is a homophone candidate
        let mutations = mutator.find_possible_mutations(text);
        assert_eq!(mutations.len(), 1); // your
        assert!(matches!(mutations[0], Mutation::ReplaceHomophone(0, 4))); // "your" starts at 0, length 4
    }

    #[test]
    fn test_find_possible_mutations_all_types() {
        let mutator = create_test_mutator(1.0, true, true, true);
        let text = "It's your text!";
        let mutations = mutator.find_possible_mutations(text);

        assert_eq!(mutations.len(), 11);
    }

    #[test]
    fn test_mutate_no_mutations_zero_rate() {
        let mut mutator = create_test_mutator(0.0, true, true, true);
        let text = "It's your text!";
        let result = mutator.mutate(text);
        assert_eq!(result.mutated_text, text);
        assert!(result.mutations.is_empty());
    }

    #[test]
    fn test_mutate_swap_only_full_rate() {
        let mut mutator = create_test_mutator(1.0, true, false, false);

        let text = "abc";

        let result = mutator.mutate(text);

        assert_eq!(result.mutated_text, "cab");
        assert_eq!(result.mutations.len(), 2);

        assert!(
            result
                .mutations
                .iter()
                .any(|m| matches!(m, Mutation::SwapLetters(0)))
        );
        assert!(
            result
                .mutations
                .iter()
                .any(|m| matches!(m, Mutation::SwapLetters(1)))
        );
    }

    #[test]
    fn test_mutate_punctuation_only_full_rate() {
        let mut mutator = create_test_mutator(1.0, false, true, false);

        let text = "a,b.c!";

        let result = mutator.mutate(text);

        assert_eq!(result.mutated_text, "abc");

        assert_eq!(result.mutations.len(), 3);

        assert!(
            result
                .mutations
                .iter()
                .any(|m| matches!(m, Mutation::RemovePunctuation(1)))
        );
        assert!(
            result
                .mutations
                .iter()
                .any(|m| matches!(m, Mutation::RemovePunctuation(3)))
        );
        assert!(
            result
                .mutations
                .iter()
                .any(|m| matches!(m, Mutation::RemovePunctuation(5)))
        );
    }

    #[test]
    fn test_mutate_homophones_only_full_rate() {
        let mut mutator = create_test_mutator(1.0, false, false, true);
        let text = "your text to test";

        let result = mutator.mutate(text);

        assert_eq!(result.mutated_text, "you're text two test");

        assert_eq!(result.mutations.len(), 2);

        assert!(
            result
                .mutations
                .iter()
                .any(|m| matches!(m, Mutation::ReplaceHomophone(0, 4)))
        );
        assert!(
            result
                .mutations
                .iter()
                .any(|m| matches!(m, Mutation::ReplaceHomophone(10, 2)))
        );
    }

    #[test]
    fn test_mutate_all_types_full_rate() {
        let mut mutator = create_test_mutator(1.0, true, true, true);
        let text = "It's your text!";

        let result = mutator.mutate(text);
        assert_eq!(result.mutations.len(), 11);
        assert_ne!(result.mutated_text, text);

        // No homophones for simplicity
        let mut mutator_simple = create_test_mutator(1.0, true, true, false);
        let text_simple = "Test.";
        let result_simple = mutator_simple.mutate(text_simple);
        assert_eq!(result_simple.mutated_text, "tTes");
        assert_eq!(result_simple.mutations.len(), 4);
    }

    #[test]
    fn test_mutate_partial_rate_fixed_seed() {
        // Rate 0.5 means roughly half the possible mutations should be chosen
        let mut mutator = create_test_mutator(0.5, true, true, true);
        let text = "It's your text!";
        let result = mutator.mutate(text);

        assert_eq!(result.mutations.len(), 5);

        assert_ne!(result.mutated_text, text);
    }

    #[test]
    fn test_mutate_empty_string() {
        let mut mutator = create_test_mutator(1.0, true, true, true);
        let text = "";
        let result = mutator.mutate(text);
        assert_eq!(result.mutated_text, "");
        assert!(result.mutations.is_empty());
    }

    #[test]
    fn test_mutate_no_possible_mutations() {
        let mut mutator = create_test_mutator(1.0, true, true, true);
        // Text with only spaces
        let text_spaces = "   ";
        let result_spaces = mutator.mutate(text_spaces);
        assert_eq!(result_spaces.mutated_text, "   ");
        assert!(result_spaces.mutations.is_empty());

        // Text with only numbers
        let text_numbers = "12345";
        let result_numbers = mutator.mutate(text_numbers);
        assert_eq!(result_numbers.mutated_text, "12345");
        assert!(result_numbers.mutations.is_empty());
    }

    #[test]
    fn test_homophone_preserves_punctuation() {
        let mut mutator = create_test_mutator(1.0, false, false, true);
        let text = "Were you there?";
        let result = mutator.mutate(text);
        assert_eq!(result.mutated_text, "We're you they're?");
        assert_eq!(result.mutations.len(), 2);
    }

    #[test]
    fn test_homophone_case_preservation() {
        let mut mutator = create_test_mutator(1.0, false, false, true);
        let text = "Your car, your rules.";
        let result = mutator.mutate(text);
        assert_eq!(result.mutated_text, "You're car, you're rules.");
        assert_eq!(result.mutations.len(), 2);
    }
}
