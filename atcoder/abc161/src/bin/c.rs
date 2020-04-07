use procon::prelude::*;

fn main() {
    input! {
        n: isize,
        k: isize,
    }

    println!("{}", (n % k).min((n % k - k).abs()));
}
