use proconio::input;

fn main() {
    input! {
        a: isize, b: isize,
    }

    println!("{}", (a+b).max(a-b).max(a*b))
}
