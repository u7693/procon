use proconio::input;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    }

    let r = if (b / k) - (a / k) >= 1 || a % k == 0 || b % k == 0 {
        "OK"
    } else {
        "NG"
    };

    println!("{}", r);
}
