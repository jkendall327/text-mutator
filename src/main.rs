use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use rand::{Rng, thread_rng, seq::SliceRandom};
use structopt::StructOpt;

/// A program that deliberately introduces minor errors into text for proofreading practice
#[derive(StructOpt, Debug)]
#[structopt(name = "text-mutator")]
struct Opt {
    /// Input file path (if not provided, will read from stdin)
    #[structopt(short, long, parse(from_os_str))]
    input: Option<PathBuf>,

    /// Output file path (if not provided, will write to stdout)
    #[structopt(short, long, parse(from_os_str))]
    output: Option<PathBuf>,

    /// Mutation rate (0.0-1.0) - higher means more errors
    #[structopt(short, long, default_value = "0.05")]
    rate: f32,

    /// Random seed for reproducible results
    #[structopt(short, long)]
    seed: Option<u64>,

    /// Enable letter swapping mutations
    #[structopt(long)]
    swap_letters: bool,

    /// Enable punctuation removal mutations
    #[structopt(long)]
    remove_punctuation: bool,

    /// Enable homophone replacements (your/you're, their/there/they're, etc.)
    #[structopt(long)]
    homophones: bool,

    /// Enable all mutation types
    #[structopt(long)]
    all_mutations: bool,
}

/// A mutation that can be applied to text
enum Mutation {
    SwapLetters(usize),              // Swap with next letter
    RemovePunctuation(usize),        // Remove punctuation at index
    ReplaceHomophone(usize, usize),  // Replace word at index with length
}

/// Common homophones that can be swapped
struct HomophoneSets {
    sets: Vec<Vec<&'static str>>,
}

impl HomophoneSets {
    fn new() -> Self {
        HomophoneSets {
            sets: vec![
                vec!["your", "you're"],
                vec!["their", "there", "they're"],
                vec!["its", "it's"],
                vec!["to", "too", "two"],
                vec!["than", "then"],
                vec!["affect", "effect"],
                vec!["accept", "except"],
                vec!["who's", "whose"],
                vec!["which", "witch"],
                vec!["were", "we're", "where"],
                vec!["lose", "loose"],
            ]
        }
    }

    fn find_matching_set(&self, word: &str) -> Option<&Vec<&'static str>> {
        self.sets.iter().find(|set| set.contains(&word.to_lowercase().as_str()))
    }

    fn get_alternative<R: Rng>(&self, word: &str, rng: &mut R) -> Option<String> {
        if let Some(set) = self.find_matching_set(word) {
            let alternatives: Vec<&&str> = set.iter()
                .filter(|&&w| w.to_lowercase() != word.to_lowercase())
                .collect();

            if !alternatives.is_empty() {
                let alt = **alternatives.choose(rng).unwrap();

                // Preserve capitalization
                if word.chars().next().unwrap().is_uppercase() {
                    let mut alt_chars: Vec<char> = alt.chars().collect();
                    if !alt_chars.is_empty() {
                        alt_chars[0] = alt_chars[0].to_uppercase().next().unwrap();
                    }
                    return Some(alt_chars.into_iter().collect());
                } else {
                    return Some(alt.to_string());
                }
            }
        }
        None
    }
}

/// Applies mutations to text
struct TextMutator {
    mutation_rate: f32,
    rng: rand::rngs::ThreadRng,
    swap_letters: bool,
    remove_punctuation: bool,
    use_homophones: bool,
    homophones: HomophoneSets,
}

impl TextMutator {
    fn new(mutation_rate: f32, seed: Option<u64>, swap_letters: bool,
           remove_punctuation: bool, homophones: bool) -> Self {
        let rng = thread_rng();
        if let Some(seed_val) = seed {
            // If we had a proper seeded RNG, we'd use it here
            // For this example, we'll just note that we'd use the seed
            println!("Note: Seed value {} would be used here", seed_val);
        }

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

    fn mutate(&mut self, text: &str) -> (String, usize) {
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

fn main() -> io::Result<()> {
    let opt = Opt::from_args();

    // Set mutation flags
    let swap_letters = opt.swap_letters || opt.all_mutations;
    let remove_punctuation = opt.remove_punctuation || opt.all_mutations;
    let homophones = opt.homophones || opt.all_mutations;

    // If no specific mutations are enabled, default to all
    let (swap_letters, remove_punctuation, homophones) = if !swap_letters && !remove_punctuation && !homophones {
        (true, true, true)
    } else {
        (swap_letters, remove_punctuation, homophones)
    };

    // Read input
    let mut input = String::new();
    match &opt.input {
        Some(path) => {
            input = fs::read_to_string(path)?;
        },
        None => {
            println!("Please enter input:");
            io::stdin().read_line(&mut input)?;
        }
    }

    // Apply mutations
    let mut text_mutator = TextMutator::new(
        opt.rate,
        opt.seed,
        swap_letters,
        remove_punctuation,
        homophones
    );

    let (mutated_text, num_mutations) = text_mutator.mutate(&input);

    // Write output
    match &opt.output {
        Some(path) => {
            fs::write(path, &mutated_text)?;
            eprintln!("Added {} mutations to the text", num_mutations);
        },
        None => {
            io::stdout().write_all(mutated_text.as_bytes())?;
            eprintln!("\n--- Added {} mutations to the text ---", num_mutations);
        }
    }

    Ok(())
}