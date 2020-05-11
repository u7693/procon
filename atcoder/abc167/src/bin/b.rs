use proconio::input;

fn main() {
    input! {
        a: isize,
        b: isize,
        c: isize,
        k: isize,
    }
    let mut r = 0;
    r += a.min(k);
    if k - a - b > 0 {
        r -= k - a - b;
    }
    println!("{}", r);
}
