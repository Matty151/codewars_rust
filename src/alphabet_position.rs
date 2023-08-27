pub fn main() {
    println!("{}", alphabet_position("The sunset sets at twelve o' clock."));
}

fn alphabet_position(text: &str) -> String {
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars();
    let mut alphabet_mut = alphabet.clone();
    let mut result = String::new();

    for char in text.chars() {
        let index = alphabet_mut.position(|letter| letter == char);
        alphabet_mut = alphabet.clone();

        if index != None {
            let mut index = index.unwrap() + 1;

            if index > 26 {
                index -= 26;
            }

            result.push_str(&index.to_string());
            result.push_str(" ");
        }
    }

    result.trim_end().to_string()
}