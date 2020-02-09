use procon::prelude::*;

fn main() {
    input! {
        h: usize,
        n: usize,
        a: [usize; n],
    }

    let s: usize = a.iter().sum();
    let ans = if s >= h {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
