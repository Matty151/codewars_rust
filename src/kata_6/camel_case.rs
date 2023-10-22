pub fn main() {
    println!("{:?}", to_camel_case("the-stealth-warrior"));
}

fn to_camel_case(text: &str) -> String {
    if text.is_empty() {
        return String::new();
    }

    let mut result = String::new();

    for mut part in text.split(['_', '-']).map(|part| part.to_string()) {
        let camel_case = format!("{}{part}", part.remove(0).to_uppercase());

        result += &camel_case;
    }

    let first = text.chars().next().unwrap();

    let first_char = match first.is_uppercase() {
        true => result.remove(0),
        false => result.remove(0).to_lowercase().next().unwrap()
    };

    format!("{}{result}", first_char)
}
