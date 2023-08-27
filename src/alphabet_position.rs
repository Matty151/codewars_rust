const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";

pub fn main() {
    println!("{}", alphabet_position("The sunset sets at twelve o' clock."));
}

fn alphabet_position(text: &str) -> String {
    let lower_text = text.to_lowercase();
    let mut result = Vec::new();

    for char in lower_text.chars() {
        if let Some(index) = ALPHABET.chars().position(|letter| letter == char) {
            result.push((index + 1).to_string());
        }
    }

    result.join(" ")
}