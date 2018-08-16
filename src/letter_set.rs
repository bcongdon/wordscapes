use std::collections::HashSet;
use util;

type Counts = [i32; 256];

struct LetterSet {
    counts: Counts,
}

impl LetterSet {
    fn build_counts(string: &str, counts: &mut Counts) {
        for i in 0..256 {
            counts[i] = 0;
        }
        for ch in string.chars() {
            counts[ch as usize] += 1;
        }
    }

    fn from_string(string: &str) -> LetterSet {
        let mut counts: Counts = [0; 256];
        LetterSet::build_counts(string, &mut counts);
        LetterSet { counts }
    }

    #[inline]
    fn can_build(&mut self, string: &str) -> bool {
        let mut other = [0; 256];
        LetterSet::build_counts(string, &mut other);

        other
            .iter()
            .enumerate()
            .all(|(idx, count)| self.counts[idx] >= *count)
    }
}

pub fn letter_set_naive(letters: String) -> HashSet<String> {
    let mut letter_set = LetterSet::from_string(&letters);

    util::DICTIONARY
        .split("\n")
        .filter(|line| line.len() >= 3 && letter_set.can_build(&line))
        .map(String::from)
        .collect()
}

pub fn letter_set_faster(letters: String) -> HashSet<String> {
    let mut letter_set = LetterSet::from_string(&letters);

    let letters_set = util::make_letter_set(letters);
    util::DICTIONARY
        .split("\n")
        .filter(|line| {
            line.len() >= 3
                && line.as_bytes().iter().all(|b| letters_set.contains(b))
                && letter_set.can_build(&line)
        }).map(String::from)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let mut letter_set = LetterSet::from_string("aabbcc");

        assert!(letter_set.can_build("a"));
        assert!(letter_set.can_build("cab"));
        assert!(letter_set.can_build("cabcab"));
        assert!(letter_set.can_build(""));
        assert!(!letter_set.can_build("d"));
        assert!(!letter_set.can_build("cabd"));
    }
}
