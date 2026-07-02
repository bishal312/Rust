mod emoji_mapping;

/// converts words in a given sentences to their corresponding emojis.
/// 
/// # Arguments
/// 
/// * `text` - A string slice that holds the sentence to convert
/// 
/// # Returns
/// 
/// A new `String` with matching words replaced by emojis
/// 
/// # Examples
/// 
/// ```
/// use text_to_emoji::convert_to_emojis;
/// 
/// let result = convert_to_emojis("I look sun and moon");
/// assert_eq!(result, "I look ☀️ and 🌙");
/// 
/// ```
pub fn convert_to_emojis(text: &str) -> String {
    let emoji_map = emoji_mapping::get_emoji_map();

    text.split_whitespace()
        .map(|word| emoji_map.get(word).unwrap_or(&word).to_string())
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_convert_to_emojis() {
        let input: &str = "I look sun and moon";
        let expected: &str = "I look ☀️ and 🌙";

        assert_eq!(convert_to_emojis(input), expected);
    }

    #[test]
    fn test_no_emoji() {
        let input: &str = "I look money and rich";
        let expected: &str = "I look money and rich";

        assert_eq!(convert_to_emojis(input), expected);
    }
}