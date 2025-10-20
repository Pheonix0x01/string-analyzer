#[cfg(test)]
mod tests {
    use crate::services::analyzer::analyze_string;

    #[test]
    fn test_length() {
        let result = analyze_string("hello");
        assert_eq!(result.length, 5);
    }

    #[test]
    fn test_palindrome_simple() {
        let result = analyze_string("racecar");
        assert_eq!(result.is_palindrome, true);
    }

    #[test]
    fn test_palindrome_case_insensitive() {
        let result = analyze_string("RaceCar");
        assert_eq!(result.is_palindrome, true);
    }

    #[test]
    fn test_not_palindrome() {
        let result = analyze_string("hello");
        assert_eq!(result.is_palindrome, false);
    }

    #[test]
    fn test_unique_characters() {
        let result = analyze_string("hello");
        assert_eq!(result.unique_characters, 4);
    }

    #[test]
    fn test_word_count_single() {
        let result = analyze_string("hello");
        assert_eq!(result.word_count, 1);
    }

    #[test]
    fn test_word_count_multiple() {
        let result = analyze_string("hello world test");
        assert_eq!(result.word_count, 3);
    }

    #[test]
    fn test_word_count_multiple_spaces() {
        let result = analyze_string("hello    world");
        assert_eq!(result.word_count, 2);
    }

    #[test]
    fn test_sha256_consistency() {
        let result1 = analyze_string("test");
        let result2 = analyze_string("test");
        assert_eq!(result1.sha256_hash, result2.sha256_hash);
    }

    #[test]
    fn test_character_frequency() {
        let result = analyze_string("hello");
        assert_eq!(result.character_frequency_map.get(&'h'), Some(&1));
        assert_eq!(result.character_frequency_map.get(&'e'), Some(&1));
        assert_eq!(result.character_frequency_map.get(&'l'), Some(&2));
        assert_eq!(result.character_frequency_map.get(&'o'), Some(&1));
    }
}