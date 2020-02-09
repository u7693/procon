use procon::prelude::*;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut h: [usize; n],
    }
    h.sort();

    let attack:usize = if n as isize - k as isize <= 0 {
        0
    } else {
        h[0..n-k].iter().sum()
    };

    println!("{}", attack);
}
