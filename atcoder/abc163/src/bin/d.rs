use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut result = 0;

    for x in k..=(n+1) {
        let x_min = ((x - 1) as f64 / 2.0 * x as f64) as usize;
        let x_max = x * n - x_min;
        let x_len = x_max - x_min + 1;
        result += x_len;
        result %= 1000000000 + 7;
    }

    println!("{}", result);
}
