use proconio::input;

fn main() {
    input! {
        n: isize, l: isize,
    }
    let mut apples = Vec::new();
    for i in 0..n {
        apples.push(l + i);
    }
    apples.sort_by_key(|k| k.abs());
    let mut r = 0;
    for i in 1..n {
        r += apples[i as usize];
    }
    println!("{}", r);
}
