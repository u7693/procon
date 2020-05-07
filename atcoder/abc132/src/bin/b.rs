use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }
    let mut r = 0;
    for i in 1..(n-1) {
        if p[i-1] <= p[i] && p[i] <= p[i+1]
        || p[i-1] >= p[i] && p[i] >= p[i+1] {
            r += 1;
        }
    }
    println!("{}", r);
}
