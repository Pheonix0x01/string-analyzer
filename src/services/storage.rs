use crate::models::string::{StoredString, Filters};
use std::sync::Mutex;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use sha2::{Sha256, Digest};

pub static STORE: Lazy<Mutex<HashMap<String, StoredString>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

pub fn save_string(string: StoredString) -> Result<(), String> {
    let mut store = STORE.lock().unwrap();
    if store.contains_key(&string.id) {
        return Err("String already exists".to_string());
    }
    store.insert(string.id.clone(), string);
    Ok(())
}

pub fn get_string_by_hash(hash: &str) -> Option<StoredString> {
    let store = STORE.lock().unwrap();
    store.get(hash).cloned()
}

pub fn get_string_by_value(value: &str) -> Option<StoredString> {
    let hash = compute_sha256(value);
    get_string_by_hash(&hash)
}

pub fn get_all_strings(filters: Filters) -> Vec<StoredString> {
    let store = STORE.lock().unwrap();
    store.values()
        .cloned()
        .filter(|s| apply_filters(s, &filters))
        .collect()
}

pub fn delete_string(value: &str) -> Result<(), String> {
    let hash = compute_sha256(value);
    let mut store = STORE.lock().unwrap();
    if store.remove(&hash).is_some() {
        Ok(())
    } else {
        Err("String not found".to_string())
    }
}

pub fn string_exists(hash: &str) -> bool {
    let store = STORE.lock().unwrap();
    store.contains_key(hash)
}

fn compute_sha256(s: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());
    format!("{:x}", hasher.finalize())
}

fn apply_filters(stored: &StoredString, filters: &Filters) -> bool {
    if let Some(is_pal) = filters.is_palindrome {
        if stored.properties.is_palindrome != is_pal {
            return false;
        }
    }
    if let Some(min) = filters.min_length {
        if stored.properties.length < min {
            return false;
        }
    }
    if let Some(max) = filters.max_length {
        if stored.properties.length > max {
            return false;
        }
    }
    if let Some(wc) = filters.word_count {
        if stored.properties.word_count != wc {
            return false;
        }
    }
    if let Some(ch) = filters.contains_character {
        if !stored.value.contains(ch) {
            return false;
        }
    }
    true
}