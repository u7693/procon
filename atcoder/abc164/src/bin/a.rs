use proconio::input;

fn main() {
    input! {
        s: usize,
        w: usize,
    }

    let r = if s <= w {
        "unsafe"
    } else {
        "safe"
    };

    println!("{}", r);
}
