use dictionary::Dictionary;
use permutohedron::Heap;
use std::collections::HashSet;
use util;

fn powerset<T>(s: &[T]) -> Vec<Vec<T>>
where
    T: Clone,
{
    (0..2usize.pow(s.len() as u32))
        .map(|i| {
            s.iter()
                .enumerate()
                .filter(|&(t, _)| (i >> t) % 2 == 1)
                .map(|(_, element)| element.clone())
                .collect()
        }).collect()
}

pub fn hashset_naive(letters: String) -> HashSet<String> {
    let all_words = Dictionary::new();

    let mut results: HashSet<String> = HashSet::new();
    for mut letter_set in powerset(letters.as_bytes()) {
        if letter_set.len() < 3 {
            continue;
        }
        Heap::new(&mut letter_set)
            .map(|permutation| String::from_utf8(permutation).expect("word"))
            .filter(|word| all_words.is_word(word))
            .for_each(|word| {
                results.insert(word);
            });
    }
    results
}

pub fn hashset_faster(letters: String) -> HashSet<String> {
    let letters_set = util::make_letter_set(letters.clone());
    let all_words = Dictionary::with_filter(|s| {
        s.as_bytes()
            .iter()
            .all(|b| s.len() >= 3 && letters_set.contains(b))
    });

    let letters = letters.as_bytes();
    let mut results: HashSet<String> = HashSet::new();
    for mut letter_set in powerset(letters) {
        if letter_set.len() < 3 {
            continue;
        }
        Heap::new(&mut letter_set)
            .map(|permutation| String::from_utf8(permutation).expect("word"))
            .filter(|word| all_words.is_word(word))
            .for_each(|word| {
                results.insert(word);
            });
    }
    results
}
