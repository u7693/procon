use procon::prelude::*;

fn main() {
    input! {
        n: u8,
        a: [i32; n],
        mut s: String,
    }

    s.push_str(", world!");

    println!("n: {}", n);
    println!("a: {:?}", a);
    println!("s: {}", s);
}
