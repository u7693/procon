use proconio::input;

fn main() {
    input! {
        n: usize,
        w: [isize; n],
    }
    let mut r = 100000;
    for i in 1..n {
        let a: isize = w[0..i].iter().sum();
        let b: isize = w[i..n].iter().sum();
        r = r.min((a-b).abs());
    }
    println!("{}", r);
}
