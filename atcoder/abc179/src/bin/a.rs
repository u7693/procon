use proconio::input;

fn main() {
    input! { s: String }
    if let Some(c) = s.chars().last() {
        if c == 's' {
            println!("{}es", s);
        } else {
            println!("{}s", s);
        }
    }
}
