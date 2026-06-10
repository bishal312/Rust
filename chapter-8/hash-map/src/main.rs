use std::collections::HashMap;

fn main() {
    println!("Hash Map");

    let mut scores: HashMap<String, i32> = HashMap::new();

    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("red"), 12);

    // overides value
    scores.insert(String::from("blue"), 190);

    // only add if doesnot exist in hashmap
    scores.entry(String::from("red")).or_insert(50);

    let team_name = String::from("blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);

    println!("HashMap = {:?}", scores);
    println!("Score = {:?}", score);

    for (key, value) in scores {
        println!("{:?} => {:?}", key, value);
    }

    // each unique key can only have one value associated with it at a time (but not vice versa: For example, both the Blue team and the Yellow team could have the value 10 stored in the scores hash map).

    map_words();
}

fn map_words() {
    println!("Map_words Function ------->");

    let text: &str = "hello world wonderful world";
    let mut map: HashMap<&str, i32> = HashMap::new();

    // for word in text.split_whitespace() {
    //     println!("Word = {:?}", word);
    // }

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}
