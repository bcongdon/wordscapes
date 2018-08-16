#![feature(use_extern_macros)]
#![feature(vec_remove_item)]

extern crate sequence_trie;
extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

mod trie;
mod util;
use trie::trie_faster;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn calculate_valid_words(name: &str) -> String {
    trie_faster(name.to_string())
        .iter()
        .cloned()
        .collect::<Vec<String>>()
        .join("\n")
}
