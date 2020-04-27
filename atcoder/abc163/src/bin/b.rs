use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let mut s = 0;
    for x in a {
        s += x;
    }

    let res = if s > n {
        -1
    } else {
        (n - s) as isize
    };

    println!("{}", res);
}
