use proconio::input;

fn main() {
    input! { n: usize }
    let mut v = Vec::with_capacity(n);
    for _ in 0..n {
        input! { d1: usize, d2: usize}
        v.push(d1 == d2);
    }
    for i in 0..(n-2) {
        if v[i] && v[i+1] && v[i+2] {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
