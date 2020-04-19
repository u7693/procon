use proconio::input;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        sp: [(String, usize); n],
    }

    let mut sp: Vec<(String, Reverse<usize>, usize)> = sp.into_iter()
                .enumerate()
                .map(|x| ((x.1).0, Reverse((x.1).1), x.0 + 1))
                .collect();

    sp.sort();

    for x in sp {
        println!("{}", x.2);
    }
}
