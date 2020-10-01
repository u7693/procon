use proconio::input;

fn main() {
    input! { d: usize, t: usize, s: usize }
    let s = if s * t >= d { "Yes" } else { "No" };
    println!("{}", s);
}
