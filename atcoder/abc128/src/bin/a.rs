use procon::prelude::*;

fn main() {
    input! {
        a: usize,
        p: usize,
    }

    println!("{}", (a * 3 + p) / 2);
}
