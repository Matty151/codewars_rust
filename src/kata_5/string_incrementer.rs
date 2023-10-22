pub fn main() {
    println!("{}", increment_string(""));
    println!("{}", increment_string("1"));
    // println!("{}", increment_string("foo"));
    // println!("{}", increment_string("foo1"));
    // println!("{}", increment_string("foobar001"));
    // println!("{}", increment_string("foobar99"));
    // println!("{}", increment_string("foobar099"));
    // println!("{}", increment_string("HereComesATrickyTest99999999999999999999999999999999999999"));
    // println!("{}", increment_string("P4fK6kCv3U1zG0cqKO89909919894497907499"));
}

fn increment_string(s: &str) -> String {
    match s.chars().rev().position(|char| !char.is_numeric()) {
        Some(index) => {
            if index == 0 {
                return format!("{s}1");
            }

            let first_number_index = s.len() - index;

            let sub = &s[..first_number_index];

            let mut number = s[first_number_index..].parse::<u128>().unwrap();

            number += 1;

            format!("{sub}{:0width$}", number, width = s.len() - first_number_index)
        }
        None => {
            if s.len() > 1 || s.chars().nth(0).unwrap().is_numeric() {
                let mut number = s.parse::<u128>().unwrap();

                number += 1;

                if s.len() > 1 {
                    format!("{:00$}{number}", s.len() - 1)
                } else {
                    format!("{number}")
                }
            } else {
                format!("{s}1")
            }
        }
    }
}
