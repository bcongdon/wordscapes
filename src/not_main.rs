#![feature(test)]
#![feature(vec_remove_item)]
extern crate permutohedron;
extern crate sequence_trie;
extern crate test;

mod dictionary;
mod hashset;
mod letter_set;
mod trie;
mod util;

use hashset::*;
use letter_set::*;
use std::env;
use trie::*;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    static INPUTS: &'static [&str] = &["brush", "candy", "allow", "pusher", "emoji"];

    #[test]
    #[ignore]
    fn agreement_naive() {
        for input in INPUTS {
            let output = hashset_naive(input.to_string());
            assert_eq!(output, trie_naive(input.to_string()));
            assert_eq!(output, letter_set_naive(input.to_string()));
        }
    }

    #[test]
    fn agreement_faster() {
        for input in INPUTS {
            let output = hashset_naive(input.to_string());
            assert_eq!(output, hashset_faster(input.to_string()));
            assert_eq!(output, trie_faster(input.to_string()));
            assert_eq!(output, letter_set_faster(input.to_string()));
        }
    }

    #[bench]
    fn bench_hashset_naive(b: &mut Bencher) {
        let mut cycler = INPUTS.iter().cycle();
        b.iter(|| hashset_naive(cycler.next().unwrap().to_string()));
    }

    #[bench]
    fn bench_hashset_faster(b: &mut Bencher) {
        let mut cycler = INPUTS.iter().cycle();
        b.iter(|| hashset_faster(cycler.next().unwrap().to_string()));
    }

    #[bench]
    fn bench_trie_naive(b: &mut Bencher) {
        let mut cycler = INPUTS.iter().cycle();
        b.iter(|| trie_naive(cycler.next().unwrap().to_string()));
    }

    #[bench]
    fn bench_trie_faster(b: &mut Bencher) {
        let mut cycler = INPUTS.iter().cycle();
        b.iter(|| trie_faster(cycler.next().unwrap().to_string()));
    }

    #[bench]
    fn bench_letter_set_naive(b: &mut Bencher) {
        let mut cycler = INPUTS.iter().cycle();
        b.iter(|| letter_set_naive(cycler.next().unwrap().to_string()));
    }

    #[bench]
    fn bench_letter_set_faster(b: &mut Bencher) {
        let mut cycler = INPUTS.iter().cycle();
        b.iter(|| letter_set_faster(cycler.next().unwrap().to_string()));
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if let Some(letters) = args.get(1) {
        println!(
            "{}",
            trie_faster(letters.to_string())
                .iter()
                .cloned()
                .collect::<Vec<String>>()
                .join("\n")
        );
    } else {
        println!("Provide a set of letters");
    }
}
