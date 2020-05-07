use proconio::input;

fn good_distance(y: &Vec<isize>, z: &Vec<isize>, d: usize) -> bool {
    let mut s = 0;
    for i in 0..d {
        s += (y[i] - z[i]).pow(2);
    }
    for i in 0..=s {
        if i * i == s {
            return true;
        }
        if i * i > s {
            return false;
        }
    }
    false
}

fn main() {
    input! {
        n: usize, d: usize,
        x: [[isize; d]; n],
    }
    let mut r = 0;
    for i in 0..(n-1) {
        for j in (i+1)..n {
            if good_distance(&x[i], &x[j], d) {
                r += 1;
            }
        }
    }
    println!("{}", r);
}
