use proconio::input;

fn main() {
    input! {
        a: isize, b: isize, c: isize,
    }

    println!("{}", (c - a + b).max(0));
}
