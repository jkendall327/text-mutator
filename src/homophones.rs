use rand::{seq::SliceRandom, Rng};

/// Common homophones that can be swapped
pub(crate) struct HomophoneSets {
    sets: Vec<Vec<&'static str>>,
}

impl HomophoneSets {
    pub(crate) fn new() -> Self {
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

    pub(crate) fn find_matching_set(&self, word: &str) -> Option<&Vec<&'static str>> {
        self.sets.iter().find(|set| set.contains(&word.to_lowercase().as_str()))
    }

    pub(crate) fn get_alternative<R: Rng>(&self, word: &str, rng: &mut R) -> Option<String> {
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