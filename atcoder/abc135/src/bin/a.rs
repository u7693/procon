use proconio::input;

fn main() {
    input! {
        a: usize, b: usize,
    }

    if (a + b) % 2 == 0 {
        println!("{}", (a + b) / 2);
    } else {
        println!("IMPOSSIBLE");
    }
}
