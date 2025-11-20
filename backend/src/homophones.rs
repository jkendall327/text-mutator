use rand::{Rng, seq::IndexedRandom};
use serde::Deserialize;

/// Common homophones that can be swapped
#[derive(Debug, Deserialize)]
pub(crate) struct HomophoneSets {
    pub(crate) sets: Vec<Vec<String>>,
}

impl HomophoneSets {
    #[cfg(test)]
    pub(crate) fn new_for_tests() -> Self {
        HomophoneSets {
            sets: vec![
                vec!["your".to_string(), "you're".to_string()],
                vec!["their".to_string(), "there".to_string(), "they're".to_string()],
                vec!["its".to_string(), "it's".to_string()],
                vec!["to".to_string(), "too".to_string(), "two".to_string()],
                vec!["than".to_string(), "then".to_string()],
                vec!["affect".to_string(), "effect".to_string()],
                vec!["accept".to_string(), "except".to_string()],
                vec!["who's".to_string(), "whose".to_string()],
                vec!["which".to_string(), "witch".to_string()],
                vec!["were".to_string(), "we're".to_string(), "where".to_string()],
                vec!["lose".to_string(), "loose".to_string()],
            ],
        }
    }

    pub(crate) fn find_matching_set(&self, word: &str) -> Option<&Vec<String>> {
        self.sets
            .iter()
            .find(|set| set.contains(&word.to_lowercase()))
    }

    pub(crate) fn get_alternative<R: Rng>(&self, word: &str, rng: &mut R) -> Option<String> {
        if let Some(set) = self.find_matching_set(word) {
            let alternatives: Vec<&String> = set
                .iter()
                .filter(|w| w.to_lowercase() != word.to_lowercase())
                .collect();

            if !alternatives.is_empty() {
                let alt = alternatives.choose(rng).unwrap();

                // Preserve capitalization
                if word.chars().next().unwrap().is_uppercase() {
                    let mut alt_chars: Vec<char> = alt.chars().collect();
                    if !alt_chars.is_empty() {
                        alt_chars[0] = alt_chars[0].to_uppercase().next().unwrap();
                    }
                    return Some(alt_chars.into_iter().collect());
                }

                return Some(alt.to_string());
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_matching_set_found() {
        let hs = HomophoneSets::new_for_tests();
        let set = hs.find_matching_set("your");
        assert!(set.is_some());
        assert!(set.unwrap().contains(&"you're".to_string()));
    }

    #[test]
    fn test_find_matching_set_case_insensitive() {
        let hs = HomophoneSets::new_for_tests();
        let set = hs.find_matching_set("Their");
        assert!(set.is_some());
        assert!(set.unwrap().contains(&"there".to_string()));
        assert!(set.unwrap().contains(&"they're".to_string()));
    }

    #[test]
    fn test_find_matching_set_not_found() {
        let hs = HomophoneSets::new_for_tests();
        let set = hs.find_matching_set("hello");
        assert!(set.is_none());
    }

    #[test]
    fn test_get_alternative_basic() {
        let hs = HomophoneSets::new_for_tests();
        let mut rng = rand::rng();
        let alt = hs.get_alternative("to", &mut rng);
        assert!(alt.is_some());
        let alt_word = alt.unwrap();
        assert!(alt_word == "too" || alt_word == "two");
    }

    #[test]
    fn test_get_alternative_case_preserved() {
        let hs = HomophoneSets::new_for_tests();
        let mut rng = rand::rng();
        // Test "Your" -> "You're"
        let alt_your = hs.get_alternative("Your", &mut rng);
        assert!(alt_your.is_some());
        assert_eq!(alt_your.unwrap(), "You're");

        // Test "They're" -> "Their" or "There"
        let alt_theyre = hs.get_alternative("They're", &mut rng);
        assert!(alt_theyre.is_some());
        let alt_word = alt_theyre.unwrap();
        assert!(alt_word == "Their" || alt_word == "There");
        assert!(alt_word.chars().next().unwrap().is_uppercase());
    }

    #[test]
    fn test_get_alternative_no_match() {
        let hs = HomophoneSets::new_for_tests();
        let mut rng = rand::rng();
        let alt = hs.get_alternative("world", &mut rng);
        assert!(alt.is_none());
    }

    #[test]
    fn test_get_alternative_single_option() {
        let hs = HomophoneSets::new_for_tests();
        let mut rng = rand::rng();
        // Test "affect" -> "effect"
        let alt_affect = hs.get_alternative("affect", &mut rng);
        assert!(alt_affect.is_some());
        assert_eq!(alt_affect.unwrap(), "effect");

        // Test "Effect" -> "Affect" (case preserved)
        let alt_effect_caps = hs.get_alternative("Effect", &mut rng);
        assert!(alt_effect_caps.is_some());
        assert_eq!(alt_effect_caps.unwrap(), "Affect");
    }
}
