use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut x = 0;
    for i in 1..=n {
        if i % 2 != 0 {
            x += 1;
        }
    }

    println!("{}", x as f64 / n as f64);
}
