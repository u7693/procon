use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();
    for x in s {
        map.insert(x, true);
    }
    println!("{}", map.len());
}
