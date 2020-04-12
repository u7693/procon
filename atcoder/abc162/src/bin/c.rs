use procon::prelude::*;

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

fn main() {
    input! {
        k: usize,
    }

    let mut r = 0;

    for a in 1..=k {
        for b in 1..=k {
            for c in 1..=k {
                r += gcd(a, gcd(b, c));
            }
        }
    }

    println!("{}", r);
}
