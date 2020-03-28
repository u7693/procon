use procon::prelude::*;

fn main() {
    input!{
        s: String,
    }

    let s: Vec<char> = s.chars().collect();
    let r = if s[2] == s[3] && s[4] == s[5] {
        "Yes"
    } else {
        "No"
    };
    println!("{}", r);
}
