fn shortest_word(sentence: &str) -> Option<&str> {
    sentence
        .split_whitespace()
        .min_by_key(|word| word.len())
}

fn main() {
    let input = "Hi, This is shristi here.";
    match shortest_word(input) {
        Some(shortest) => println!("The shortest word is: {}", shortest),
        None => println!("No words found in the input."),
    }
}
