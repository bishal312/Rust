fn main() {
    println!("String UTF-8");

    let hello = String::from("नमस्ते");

    // नमस्ते
    // If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is stored as a vector of u8 values that looks like this:

    
    // Bytes
    // [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
    // 224, 165, 135]
    // That’s 18 bytes and is how computers ultimately store this data. If we look at them as Unicode scalar values, which are what Rust’s char type is, those bytes look like this:
    
    // for i in hello.as_bytes() {
    //     println!("i = {i}");
    // }

    // Scaler
    // ['न', 'म', 'स', '्', 'त', 'े']

    for i in hello.chars() {
        println!("i = {i}");
    }
    

    // grapheme cluster
    // ["न", "म", "स्", "ते"]

    println!("{hello}");
}
