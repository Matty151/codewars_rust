pub fn main() {
    println!("{:?}", split_string("abcdef"));
    println!("{:?}", split_string("abcdefg"));
    println!("{:?}", split_string("abcdefgh"));
}

fn split_string(string: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();
    let chars: Vec<char> = string.chars().collect();
    let mut temp_string = String::new();

    for (i, &char) in chars.iter().enumerate() {
        temp_string.push(char);

        if (i + 1) % 2 == 0 {
            result.push(temp_string.clone());
            temp_string = String::new();
        } else if i == chars.len() - 1 {
            if temp_string.len() == 1 {
                temp_string.push('_');
            }

            result.push(temp_string.clone());
        }
    }

    result
}