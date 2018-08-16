use std::collections::HashSet;
use util;

pub struct Dictionary {
    words: HashSet<String>,
}

impl Dictionary {
    pub fn new() -> Dictionary {
        Dictionary::with_filter(|_| true)
    }

    pub fn with_filter<F>(filter: F) -> Dictionary
    where
        F: Fn(String) -> bool,
    {
        Dictionary {
            words: util::DICTIONARY
                .split("\n")
                .filter(|s| filter(s.to_string()))
                .map(String::from)
                .collect(),
        }
    }

    pub fn is_word(&self, w: &str) -> bool {
        self.words.contains(w)
    }
}
