use procon::prelude::*;

fn main() {
    input! {
        n: usize,
    }

    let mut r = 0;

    for i in 1..=n {
        if i % 3 != 0 && i % 5 != 0 {
            r += i;
        }
    }

    println!("{}", r);
}
