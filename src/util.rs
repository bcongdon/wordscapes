use std::collections::HashSet;

pub const DICTIONARY: &str = include_str!("words.txt");

pub fn make_letter_set(letters: String) -> HashSet<u8> {
    letters
        .as_bytes()
        .iter()
        .map(|c| c.clone())
        .collect::<HashSet<u8>>()
}
