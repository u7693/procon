use proconio::input;

fn main() {
    input! {
        n: usize, a: usize, b: usize
    }

    println!("{}", (a * n).min(b));
}
