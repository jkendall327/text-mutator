use rand::{seq::SliceRandom, Rng, SeedableRng};
use rand::rngs::StdRng;

use crate::homophones::HomophoneSets;

/// A mutation that can be applied to text
enum Mutation {
    SwapLetters(usize),              // Swap with next letter
    RemovePunctuation(usize),        // Remove punctuation at index
    ReplaceHomophone(usize, usize),  // Replace word at index with length
}

/// Applies mutations to text
pub struct TextMutator {
    mutation_rate: f32,
    rng: StdRng,
    swap_letters: bool,
    remove_punctuation: bool,
    use_homophones: bool,
    homophones: HomophoneSets,
}

impl TextMutator {
    pub(crate) fn new(
        mutation_rate: f32,
        seed: Option<u64>,
        swap_letters: bool,
        remove_punctuation: bool,
        homophones: bool) -> Self {

        let rng = match seed {
            Some(seed_val) => StdRng::seed_from_u64(seed_val),
            None => StdRng::from_entropy(),
        };

        TextMutator {
            mutation_rate,
            rng,
            swap_letters,
            remove_punctuation,
            use_homophones: homophones,
            homophones: HomophoneSets::new(),
        }
    }

    fn find_possible_mutations(&self, text: &str) -> Vec<Mutation> {
        let mut mutations = Vec::new();
        let chars: Vec<char> = text.chars().collect();

        // Find possible letter swaps
        if self.swap_letters {
            for i in 0..chars.len().saturating_sub(1) {
                if chars[i].is_alphabetic() && chars[i+1].is_alphabetic() {
                    mutations.push(Mutation::SwapLetters(i));
                }
            }
        }

        // Find punctuation that could be removed
        if self.remove_punctuation {
            for (i, c) in chars.iter().enumerate() {
                if c.is_ascii_punctuation() {
                    mutations.push(Mutation::RemovePunctuation(i));
                }
            }
        }

        // Find homophones that could be replaced
        if self.use_homophones {
            let words: Vec<&str> = text.split_whitespace().collect();
            let mut char_index = 0;

            for word in words {
                // Skip past whitespace to get to the word
                while char_index < text.len() && !text[char_index..].starts_with(word) {
                    char_index += 1;
                }

                // Strip punctuation from word for homophone lookup
                let clean_word: String = word.chars()
                    .filter(|c| c.is_alphabetic() || c == &'\'')
                    .collect();

                if !clean_word.is_empty() {
                    if self.homophones.find_matching_set(&clean_word).is_some() {
                        mutations.push(Mutation::ReplaceHomophone(char_index, word.len()));
                    }
                }

                char_index += word.len();
            }
        }

        mutations
    }

    pub(crate) fn mutate(&mut self, text: &str) -> (String, usize) {
        let possible_mutations = self.find_possible_mutations(text);
        let num_mutations = (possible_mutations.len() as f32 * self.mutation_rate) as usize;

        if possible_mutations.is_empty() || num_mutations == 0 {
            return (text.to_string(), 0);
        }

        // Select which mutations to apply
        let mut selected_mutations = possible_mutations;
        selected_mutations.shuffle(&mut self.rng);
        selected_mutations.truncate(num_mutations);

        // Sort by position to apply from end to beginning (to avoid index shifts)
        selected_mutations.sort_by(|a, b| {
            let pos_a = match a {
                Mutation::SwapLetters(i) => *i,
                Mutation::RemovePunctuation(i) => *i,
                Mutation::ReplaceHomophone(i, _) => *i,
            };

            let pos_b = match b {
                Mutation::SwapLetters(i) => *i,
                Mutation::RemovePunctuation(i) => *i,
                Mutation::ReplaceHomophone(i, _) => *i,
            };

            pos_b.cmp(&pos_a)  // Reverse order
        });

        // Apply mutations
        let mut result = text.to_string();
        let mut actual_mutations = 0;

        for mutation in selected_mutations {
            match mutation {
                Mutation::SwapLetters(i) => {
                    let mut chars: Vec<char> = result.chars().collect();
                    if i + 1 < chars.len() {
                        chars.swap(i, i + 1);
                        result = chars.into_iter().collect();
                        actual_mutations += 1;
                    }
                },
                Mutation::RemovePunctuation(i) => {
                    let mut chars: Vec<char> = result.chars().collect();
                    if i < chars.len() && chars[i].is_ascii_punctuation() {
                        chars.remove(i);
                        result = chars.into_iter().collect();
                        actual_mutations += 1;
                    }
                },
                Mutation::ReplaceHomophone(i, len) => {
                    if i + len <= result.len() {
                        let word = &result[i..i+len];
                        let clean_word: String = word.chars()
                            .filter(|c| c.is_alphabetic() || c == &'\'')
                            .collect();

                        if let Some(alternative) = self.homophones.get_alternative(&clean_word, &mut self.rng) {
                            // Preserve trailing punctuation if any
                            let trailing_punct: String = word.chars()
                                .filter(|c| c.is_ascii_punctuation())
                                .collect();

                            let replacement = alternative + &trailing_punct;
                            result = result[..i].to_string() + &replacement + &result[i+len..];
                            actual_mutations += 1;
                        }
                    }
                }
            }
        }

        (result, actual_mutations)
    }
}
