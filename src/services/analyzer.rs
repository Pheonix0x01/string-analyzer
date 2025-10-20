use crate::models::string::StringProperties;
use sha2::{Sha256, Digest};
use std::collections::HashMap;

pub fn analyze_string(value: &str) -> StringProperties {
    let length = value.len();
    let is_palindrome = check_palindrome(value);
    let unique_characters = count_unique_characters(value);
    let word_count = count_words(value);
    let sha256_hash = compute_sha256(value);
    let character_frequency_map = build_frequency_map(value);

    StringProperties {
        length,
        is_palindrome,
        unique_characters,
        word_count,
        sha256_hash,
        character_frequency_map,
    }
}

fn check_palindrome(s: &str) -> bool {
    let cleaned: String = s.to_lowercase().chars().filter(|c| !c.is_whitespace()).collect();
    cleaned == cleaned.chars().rev().collect::<String>()
}

fn count_unique_characters(s: &str) -> usize {
    s.chars().collect::<std::collections::HashSet<_>>().len()
}

fn count_words(s: &str) -> usize {
    s.split_whitespace().count()
}

fn compute_sha256(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn build_frequency_map(s: &str) -> HashMap<char, usize> {
    let mut map = HashMap::new();
    for ch in s.chars() {
        *map.entry(ch).or_insert(0) += 1;
    }
    map
}