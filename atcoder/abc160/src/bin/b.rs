use procon::prelude::*;

fn main() {
    input!{
        x: usize,
    }
    let r = x / 500 * 1000 + (x % 500) / 5 * 5;
    println!("{}", r);
}
