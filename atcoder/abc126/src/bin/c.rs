use procon::prelude::*;

fn main() {
    input! {
        n: usize,
        k: usize,
    }
    let mut r: f64 = 0.0;
    for i in 1..=n {
        let mut c = 0;
        while i * 2_usize.pow(c) < k {
            c += 1;
        }
        r += 1.0 / (n * 2_usize.pow(c)) as f64;
    }
    println!("{}", r);
}
