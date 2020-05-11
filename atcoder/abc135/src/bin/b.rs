use proconio::input;
use std::mem;

fn main() {
    input! {
        n: usize, p: [usize; n],
    }

    let mut p_clone = p.clone();
    let mut p_sorted = p.clone();
    p_sorted.sort();

    for i in 0..(n-1) {
        for j in i..n {
            let p_clone_j = p_clone[j];
            p_clone[j] = p_clone[i];
            p_clone[i] = p_clone_j;
            if p_clone == p_sorted {
                println!("YES");
                return;
            }
            p_clone = p.clone();
        }
    }

    println!("NO");
}
