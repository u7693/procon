use proconio::input;

fn main() {
    input! {
        a: usize, mut b: usize,
    }
    let mut x = 1;
    let mut r = 0;
    while x < b {
        x += a - 1;
        r += 1;
    }
    println!("{}", r);
}
