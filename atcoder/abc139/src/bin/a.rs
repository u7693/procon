use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars, t: Chars,
    }
    let mut r = 0;
    for i in 0..3 {
        if s[i] == t[i] {
            r += 1;
        }
    }
    println!("{}", r);
}
