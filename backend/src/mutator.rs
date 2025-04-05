use rand::{seq::SliceRandom, SeedableRng};
use rand::rngs::StdRng;
use tracing::{debug, info, trace};

use crate::homophones::HomophoneSets;

/// A mutation that can be applied to text
enum Mutation {
    SwapLetters(usize),              // Swap with next letter
    RemovePunctuation(usize),        // Remove punctuation at index
    ReplaceHomophone(usize, usize),  // Replace word at index with length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_text_mutator() {
        let mutator = TextMutator::new(0.5, Some(42), true, true, true);
        assert_eq!(mutator.mutation_rate, 0.5);
        assert!(mutator.swap_letters);
        assert!(mutator.remove_punctuation);
        assert!(mutator.use_homophones);
    }

    #[test]
    fn test_find_possible_mutations_empty_text() {
        let mutator = TextMutator::new(0.5, Some(42), true, true, true);
        let mutations = mutator.find_possible_mutations("");
        assert!(mutations.is_empty());
    }

    #[test]
    fn test_find_possible_mutations_swap_letters() {
        let mutator = TextMutator::new(0.5, Some(42), true, false, false);
        let mutations = mutator.find_possible_mutations("abc");
        assert_eq!(mutations.len(), 2); // 'a' can swap with 'b', 'b' can swap with 'c'
    }

    #[test]
    fn test_find_possible_mutations_punctuation() {
        let mutator = TextMutator::new(0.5, Some(42), false, true, false);
        let mutations = mutator.find_possible_mutations("a,b.c");
        assert_eq!(mutations.len(), 2); // ',' and '.' can be removed
    }

    #[test]
    fn test_mutate_no_mutations() {
        let mut mutator = TextMutator::new(0.0, Some(42), true, true, true);
        let (result, count) = mutator.mutate("Hello, world!");
        assert_eq!(result, "Hello, world!");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_mutate_with_seed() {
        // Using a fixed seed should give deterministic results
        let mut mutator1 = TextMutator::new(1.0, Some(42), true, true, false);
        let (result1, count1) = mutator1.mutate("Hello, world!");
        
        let mut mutator2 = TextMutator::new(1.0, Some(42), true, true, false);
        let (result2, count2) = mutator2.mutate("Hello, world!");
        
        assert_eq!(result1, result2);
        assert_eq!(count1, count2);
    }

    #[test]
    fn test_mutate_swap_letters() {
        let mut mutator = TextMutator::new(1.0, Some(42), true, false, false);
        let (result, count) = mutator.mutate("abcdef");
        assert_ne!(result, "abcdef");
        assert!(count > 0);
    }

    #[test]
    fn test_mutate_remove_punctuation() {
        let mut mutator = TextMutator::new(1.0, Some(42), false, true, false);
        let (result, count) = mutator.mutate("Hello, world!");
        assert_ne!(result, "Hello, world!");
        assert!(count > 0);
        // The result should have fewer punctuation marks
        assert!(result.chars().filter(|c| c.is_ascii_punctuation()).count() < 
               "Hello, world!".chars().filter(|c| c.is_ascii_punctuation()).count());
    }

    #[test]
    fn test_mutate_homophones() {
        let mut mutator = TextMutator::new(1.0, Some(42), false, false, true);
        let (result, count) = mutator.mutate("Your book is there.");
        assert_ne!(result, "Your book is there.");
        assert!(count > 0);
    }

    #[test]
    fn test_mutation_rate_scaling() {
        let text = "The quick brown fox jumps over the lazy dog.";
        
        let mut low_rate_mutator = TextMutator::new(0.1, Some(42), true, true, true);
        let (_, low_count) = low_rate_mutator.mutate(text);
        
        let mut high_rate_mutator = TextMutator::new(0.9, Some(42), true, true, true);
        let (_, high_count) = high_rate_mutator.mutate(text);
        
        assert!(high_count > low_count, "Higher mutation rate should produce more mutations");
    }
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

        info!("Creating TextMutator with mutation_rate={}", mutation_rate);
        debug!("Mutation flags: swap_letters={}, remove_punctuation={}, homophones={}", 
               swap_letters, remove_punctuation, homophones);

        let rng = match seed {
            Some(seed_val) => {
                debug!("Using provided seed: {}", seed_val);
                StdRng::seed_from_u64(seed_val)
            },
            None => {
                debug!("Using entropy-based seed");
                StdRng::from_entropy()
            },
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
        trace!("Finding possible mutations in text of length {}", text.len());
        let mut mutations = Vec::new();
        let chars: Vec<char> = text.chars().collect();

        // Find possible letter swaps
        if self.swap_letters {
            trace!("Looking for possible letter swaps");
            for i in 0..chars.len().saturating_sub(1) {
                if chars[i].is_alphabetic() && chars[i+1].is_alphabetic() {
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
                let clean_word: String = word.chars()
                    .filter(|c| c.is_alphabetic() || c == &'\'')
                    .collect();

                if !clean_word.is_empty() {
                    if self.homophones.find_matching_set(&clean_word).is_some() {
                        trace!("Found homophone candidate: '{}'", clean_word);
                        mutations.push(Mutation::ReplaceHomophone(char_index, word.len()));
                    }
                }

                char_index += word.len();
            }
        }

        debug!("Found {} possible mutations", mutations.len());
        mutations
    }

    pub(crate) fn mutate(&mut self, text: &str) -> (String, usize) {
        info!("Mutating text of length {}", text.len());
        let possible_mutations = self.find_possible_mutations(text);
        let num_mutations = (possible_mutations.len() as f32 * self.mutation_rate) as usize;
        debug!("Planning to apply {} mutations out of {} possible", num_mutations, possible_mutations.len());

        if possible_mutations.is_empty() || num_mutations == 0 {
            info!("No mutations to apply");
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
        debug!("Applying mutations from end to beginning to avoid index shifts");

        for mutation in selected_mutations {
            match mutation {
                Mutation::SwapLetters(i) => {
                    let mut chars: Vec<char> = result.chars().collect();
                    if i + 1 < chars.len() {
                        trace!("Swapping letters at positions {} and {}: '{}' and '{}'", 
                               i, i+1, chars[i], chars[i+1]);
                        chars.swap(i, i + 1);
                        result = chars.into_iter().collect();
                        actual_mutations += 1;
                    }
                },
                Mutation::RemovePunctuation(i) => {
                    let mut chars: Vec<char> = result.chars().collect();
                    if i < chars.len() && chars[i].is_ascii_punctuation() {
                        trace!("Removing punctuation '{}' at position {}", chars[i], i);
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
                            trace!("Replacing homophone '{}' with '{}'", clean_word, alternative);
                            
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

        info!("Applied {} mutations", actual_mutations);
        (result, actual_mutations)
    }
}
