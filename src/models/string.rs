use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct StringInput {
    pub value: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StringProperties {
    pub length: usize,
    pub is_palindrome: bool,
    pub unique_characters: usize,
    pub word_count: usize,
    pub sha256_hash: String,
    pub character_frequency_map: HashMap<char, usize>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StoredString {
    pub id: String,
    pub value: String,
    pub properties: StringProperties,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize)]
pub struct FilteredResponse {
    pub data: Vec<StoredString>,
    pub count: usize,
    pub filters_applied: Filters,
}

#[derive(Debug, Serialize)]
pub struct InterpretedQuery {
    pub original: String,
    pub parsed_filters: Filters,
}

#[derive(Debug, Serialize)]
pub struct NaturalLanguageResponse {
    pub data: Vec<StoredString>,
    pub count: usize,
    pub interpreted_query: InterpretedQuery,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Filters {
    pub is_palindrome: Option<bool>,
    pub min_length: Option<usize>,
    pub max_length: Option<usize>,
    pub word_count: Option<usize>,
    pub contains_character: Option<char>,
}