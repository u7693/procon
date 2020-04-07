use procon::prelude::*;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    }
    a.sort();
    a.reverse();
    let sum = a.iter().fold(0, |acc, x| acc + x);
    for i in 0..m {
        if a[i] * 4 * m < sum {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
