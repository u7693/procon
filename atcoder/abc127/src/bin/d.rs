use procon::prelude::*;
use std::cmp::Reverse;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
        mut bc:[[usize; 2]; m],
    }

    a.sort();
    bc.sort_by_key(|k| Reverse(k[1]));

    let mut res = 0;
    let mut idx = 0;

    for i in 0..n {
        res += if idx < m && bc[idx][0] > 0 && bc[idx][1] > a[i] {
            bc[idx][0] -= 1;
            bc[idx][1]
        } else {
            a[i]
        };
        if idx < m && bc[idx][0] <= 0 {
            idx += 1;
        }
    }

    println!("{}", res);
}
