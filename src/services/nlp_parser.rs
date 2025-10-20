use crate::models::string::Filters;
use regex::Regex;

pub fn parse_natural_language(query: &str) -> Result<(Filters, String), String> {
    let original = query.to_string();
    let query_lower = query.to_lowercase();
    
    let mut filters = Filters::default();
    
    if query_lower.contains("palindromic") || query_lower.contains("palindrome") {
        filters.is_palindrome = Some(true);
    }
    
    if Regex::new(r"single word").ok().and_then(|re| re.captures(&query_lower)).is_some() {
        filters.word_count = Some(1);
    }
    
    if let Some(caps) = Regex::new(r"longer than (\d+)").ok().and_then(|re| re.captures(&query_lower)) {
        if let Some(num_str) = caps.get(1) {
            if let Ok(num) = num_str.as_str().parse::<usize>() {
                filters.min_length = Some(num + 1);
            }
        }
    }
    
    if let Some(caps) = Regex::new(r"shorter than (\d+)").ok().and_then(|re| re.captures(&query_lower)) {
        if let Some(num_str) = caps.get(1) {
            if let Ok(num) = num_str.as_str().parse::<usize>() {
                filters.max_length = Some(num - 1);
            }
        }
    }
    
    if query_lower.contains("first vowel") {
        filters.contains_character = Some('a');
    }
    
    if let Some(caps) = Regex::new(r"letter ([a-z])").ok().and_then(|re| re.captures(&query_lower)) {
        if let Some(letter) = caps.get(1) {
            filters.contains_character = Some(letter.as_str().chars().next().unwrap());
        }
    }
    
    if let Some(caps) = Regex::new(r"containing (?:the letter )?([a-z])").ok().and_then(|re| re.captures(&query_lower)) {
        if let Some(letter) = caps.get(1) {
            filters.contains_character = Some(letter.as_str().chars().next().unwrap());
        }
    }
    
    if let (Some(min), Some(max)) = (filters.min_length, filters.max_length) {
        if min > max {
            return Err("Conflicting filters: min_length > max_length".to_string());
        }
    }
    
    if filters.is_palindrome.is_none() && 
       filters.min_length.is_none() && 
       filters.max_length.is_none() && 
       filters.word_count.is_none() && 
       filters.contains_character.is_none() {
        return Err("Unable to parse natural language query".to_string());
    }
    
    Ok((filters, original))
}