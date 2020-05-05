use proconio::input;

fn main() {
    input! {
        a: isize, b: isize,
    }

    println!("{}", (a-2*b).max(0))
}
