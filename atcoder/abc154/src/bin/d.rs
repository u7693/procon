use procon::prelude::*;

fn main() {
    input!{
        n: usize,
        k: usize,
        mut p: [f64; n],
    }

    let mut su = Vec::with_capacity(1001);
    su.push(0.0);
    for i in 1..=1000 {
        su.push(su[i-1] + (i as f64));
    }

    for i in 0..n {
        p[i] = su[p[i] as usize] / p[i];
    }

    let mut result = 0.0;
    let mut cache = 0.0;

    for i in 0..k {
        cache += p[i];
    }
    result = cache;

    for i in k..n {
        cache = cache - p[i - k] + p[i];
        result = result.max(cache);
    }

    println!("{}", result);
}
