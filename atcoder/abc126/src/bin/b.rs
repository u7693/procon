use procon::prelude::*;

fn main() {
    input! {
        s: String,
    }
    let s: Vec<char> = s.chars().collect();
    let AA = s[0].to_digit(10).unwrap() * 10 + s[1].to_digit(10).unwrap();
    let BB = s[2].to_digit(10).unwrap() * 10 + s[3].to_digit(10).unwrap();
    let AA = 1 <= AA && AA <= 12;
    let BB = 1 <= BB && BB <= 12;
    let result = if AA && BB {
        "AMBIGUOUS"
    } else if AA {
        "MMYY"
    } else if BB {
        "YYMM"
    } else {
        "NA"
    };
    println!("{}", result);
}
