use std::collections::HashMap;

/// Count occurrences of words.
pub fn word_count(words: &str) -> HashMap<String, u32> {
    let words = words
        .to_lowercase()
        .to_string()
        .chars()
        .filter(|s| s.is_alphanumeric() || *s == ' ')
        .collect::<String>();

    let list: Vec<&str> = words.split(" ").collect();

    let mut map = HashMap::new();

    for word in list {
        *map.entry(word.to_string()).or_insert(0) += 1;
    }

    map.remove("");

    map
}
