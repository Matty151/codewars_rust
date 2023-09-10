use std::cmp::Ordering;

pub fn main() {
    // let test_string = "56 65 74 100 99 68 86 180 90";
    let test_string = "103 123 4444 99 2000";

    println!("{}", order_weight(test_string));
}

fn order_weight(s: &str) -> String {
    let mut numbers: Vec<&str> = s.split_whitespace()
        .map(|number| number.trim())
        .collect();

    numbers.sort_by(|a, b| {
        let sum_a = sum_digits(a);
        let sum_b = sum_digits(b);

        if sum_a < sum_b {
            Ordering::Less
        } else if sum_a > sum_b {
            Ordering::Greater
        } else {
            a.cmp(b)
        }
    });

    numbers.join(" ")
}

fn sum_digits(number: &str) -> i32 {
    number.chars()
        .map(|char| char.to_digit(10).unwrap() as i32)
        .sum()
}
