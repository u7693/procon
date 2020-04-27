use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n-1],
    }

    let mut b = Vec::new();
    b.resize(n, 0);
    for x in a {
        b[x - 1] += 1;
    }
    for x in b {
        println!("{}", x);
    }
}
