use proconio::input;

fn main() {
    input! {
        a: String,
        b: String,
    }
    let r = if a[0..a.len()] == b[0..a.len()] {
        "Yes"
    } else {
        "No"
    };
    println!("{}", r);
}
