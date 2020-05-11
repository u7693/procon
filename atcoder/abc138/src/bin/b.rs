use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [f64; n],
    }
    let mut sum_a = 0.0;
    let mut mul_a = 1.0;
    for i in 0..n {
        mul_a *= a[i];
    }
    for i in 0..n {
        sum_a += mul_a / a[i];
    }
    println!("{}", mul_a / sum_a);
}
