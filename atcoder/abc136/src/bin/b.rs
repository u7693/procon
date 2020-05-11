use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut r = 0;
    for i in 1..=n {
        if i.to_string().len() % 2 == 1 {
            r += 1;
        }
    }
    println!("{}", r);
}
