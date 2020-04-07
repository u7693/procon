use procon::prelude::*;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let r = if a <= 5 {
        0
    } else if a <= 12 {
        b / 2
    } else {
        b
    };
    println!("{}", r);
}
