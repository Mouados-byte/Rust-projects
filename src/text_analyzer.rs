use std::collections::HashMap;

fn is_palindrome(word: &str) -> bool {
    let chars = word.chars();
    chars.clone().rev().eq(chars)
}

fn analyse_text(text: &str) {
    let mut characters = 0;
    let mut words = 0;
    let mut lines = 0;
    let mut freq_map: HashMap<String, u16> = HashMap::new();

    for line in text.split("\n") {
        let line = line.trim();
        if line.is_empty(){
            continue;
        }
        lines = lines + 1;
        words = words + line.split_whitespace().count();
        characters = characters + line.chars().filter(|c| c.is_alphanumeric()).count();

        for word in line.split(" ") {
            if let Some(amount) = freq_map.get(word) {
                freq_map.insert(word.to_string(), amount + 1);
            } else {
                freq_map.insert(word.to_string(), 1);
            }
        }

        println!("{}", line.to_string())
    }

    for i in freq_map.keys() {
        println!("{}: {}, is_palindrome: {}", i, freq_map.get(i).unwrap(), is_palindrome(i))
    }

    println!("{lines}");
    println!("{words}");
    println!("{characters}");
}

fn main() {
    analyse_text("returruter the key-value pair corresponding to the supplied key. This is potentially useful:

for key types where non-identical keys can be considered equal;
for getting the &K stored key value from a borrowed &Q lookup key; or
for getting a reference to a key with the same lifetime as the collection.
The supplied key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed form must match those for the key type.");
}
