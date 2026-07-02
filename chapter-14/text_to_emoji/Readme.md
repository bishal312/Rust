# Text to Emoji

A Rust library to convert words in a sentence into emojis!
This library provides a fn and simple way to add emoji
representaitons to text.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
text_to_emoji = "0.1.0"
```

### Example

```rust
use text_to_emoji::convert_to_emojis;

let result = convert_to_image("I look sun and moon");
assert_eq!(result, "I look ☀️ and 🌙");
```

## License

This project is licensed under the MIT License.