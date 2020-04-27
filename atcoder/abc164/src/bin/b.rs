use proconio::input;

fn main() {
    input! {
        mut a: isize,
        b: isize,
        mut c: isize,
        d: isize,
    }

    while a > 0 && c > 0 {
        c -= b;
        if c <= 0 {
            println!("Yes");
            return;
        }

        a -= d;
        if a <= 0 {
            println!("No");
            return;
        }
    }
}
