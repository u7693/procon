use procon::prelude::*;
use std::iter;

fn main() {
    input!{
        s: String,
    }

    let result: String = iter::repeat("x").take(s.len()).collect();
    println!("{}", result);
}
