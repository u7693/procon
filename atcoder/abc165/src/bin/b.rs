use proconio::input;

fn main() {
    input! {
        x: f64,
    }

    let mut s = 100.0_f64;
    let mut r = 0;

    while s < x {
        s = (s * 1.01).trunc();
        r += 1;
    }

    println!("{}", r);
}
