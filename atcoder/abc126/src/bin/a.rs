use procon::prelude::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        s: String,
    }

    let mut s: Vec<char> = s.chars().collect();
    s[k-1] = s[k-1].to_ascii_lowercase();
    let s: String = s.into_iter().collect();
    println!("{}", s);
}
