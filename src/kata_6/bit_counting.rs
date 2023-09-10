pub fn main() {
    println!("{}", count_bits(1234))
}

fn count_bits(n: i64) -> u32 {
    n.count_ones()
}
