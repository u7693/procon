use proconio::input;

fn main() {
    input! { n: u64 }
    let mut r = 0u64;
    for i in 1..=n {
        r += n / i;
        if n % i == 0 {
            r -= 1;
        }
    }
    println!("{}", r);
}
