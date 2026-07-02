use std::collections::HashMap;

pub fn get_emoji_map() -> HashMap<&'static str, &'static str> {
    let mut map = HashMap::new();

    map.insert("happy", "😃");
    map.insert("sad", "😔");
    map.insert("love", "💖");
    map.insert("sun", "☀️");
    map.insert("moon", "🌙");

    map
}