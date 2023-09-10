pub fn main() {
    println!("{}", make_readable(3600));
}

fn make_readable(seconds: u32) -> String {
    let minutes = seconds / 60;
    let hours = minutes / 60;

    format!("{:02}:{:02}:{:02}", hours, minutes % 60, seconds % 60)
}
