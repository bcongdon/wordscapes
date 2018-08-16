use sequence_trie::*;
use std::collections::HashSet;
use util;

pub fn trie_naive(letters: String) -> HashSet<String> {
    let mut trie: SequenceTrie<u8, bool> = SequenceTrie::new();

    util::DICTIONARY.split("\n").for_each(|line| {
        trie.insert(line.as_bytes(), true);
    });

    trie_finder(
        &mut letters.as_bytes().iter().cloned().collect(),
        &mut vec![],
        &trie,
    )
}

pub fn trie_faster(letters: String) -> HashSet<String> {
    let letters_set = util::make_letter_set(letters.clone());
    let mut trie: SequenceTrie<u8, bool> = SequenceTrie::new();

    util::DICTIONARY
        .split("\n")
        .filter(|line| {
            line.as_bytes()
                .iter()
                .all(|b| line.len() >= 3 && letters_set.contains(b))
        }).for_each(|line| {
            trie.insert(line.as_bytes(), true);
        });

    trie_finder(
        &mut letters.as_bytes().iter().cloned().collect(),
        &mut vec![],
        &trie,
    )
}

fn trie_finder(
    letters: &mut Vec<u8>,
    stack: &mut Vec<u8>,
    trie: &SequenceTrie<u8, bool>,
) -> HashSet<String> {
    if letters.is_empty() {
        return HashSet::new();
    }

    let mut results: HashSet<String> = HashSet::new();

    if *trie.value().unwrap_or(&false) && stack.len() >= 3 {
        results.insert(String::from_utf8(stack.to_vec()).unwrap());
    }

    for (key, subtrie) in trie.children_with_keys() {
        if letters.contains(key) {
            stack.push(*key);
            letters.remove_item(key);

            for result in trie_finder(letters, stack, subtrie) {
                results.insert(result);
            }

            stack.pop();
            letters.push(*key);
        }
    }

    results
}
