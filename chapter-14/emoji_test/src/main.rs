use text_to_emoji::convert_to_emojis;

fn main() {
    let input: &str = "I love pizza and cake.";
    let convert_text: String = convert_to_emojis(input);

    println!("{}", convert_text);    
}
