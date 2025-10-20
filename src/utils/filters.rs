use crate::models::string::Filters;
use actix_web::web::Query;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct QueryParams {
    pub is_palindrome: Option<String>,
    pub min_length: Option<String>,
    pub max_length: Option<String>,
    pub word_count: Option<String>,
    pub contains_character: Option<String>,
}

pub fn parse_filters(query: Query<QueryParams>) -> Result<Filters, String> {
    let is_palindrome = if let Some(val) = &query.is_palindrome {
        Some(parse_bool(val)?)
    } else {
        None
    };

    let min_length = if let Some(val) = &query.min_length {
        Some(parse_usize(val)?)
    } else {
        None
    };

    let max_length = if let Some(val) = &query.max_length {
        Some(parse_usize(val)?)
    } else {
        None
    };

    let word_count = if let Some(val) = &query.word_count {
        Some(parse_usize(val)?)
    } else {
        None
    };

    let contains_character = if let Some(val) = &query.contains_character {
        Some(parse_char(val)?)
    } else {
        None
    };

    if let (Some(min), Some(max)) = (min_length, max_length) {
        if min > max {
            return Err("min_length cannot be greater than max_length".to_string());
        }
    }

    Ok(Filters {
        is_palindrome,
        min_length,
        max_length,
        word_count,
        contains_character,
    })
}

fn parse_bool(s: &str) -> Result<bool, String> {
    match s.to_lowercase().as_str() {
        "true" => Ok(true),
        "false" => Ok(false),
        _ => Err(format!("Invalid boolean value: {}", s)),
    }
}

fn parse_usize(s: &str) -> Result<usize, String> {
    s.parse::<usize>()
        .map_err(|_| format!("Invalid integer value: {}", s))
}

fn parse_char(s: &str) -> Result<char, String> {
    if s.len() == 1 {
        Ok(s.chars().next().unwrap())
    } else {
        Err("contains_character must be a single character".to_string())
    }
}