use procon::prelude::*;

fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize,
    }

    let result = if a == b && b != c
    || a == c && b != c
    || b == c && a != c {
        "Yes"
    } else {
        "No"
    };

    println!("{}", result);
}
