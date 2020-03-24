use procon::prelude::*;

fn main() {
    input!{
        s: String,
        _t: String,
        mut a: usize,
        mut b: usize,
        u: String,
    }

    if s == u {
        a -= 1;
    } else {
        b -= 1;
    }

    println!("{} {}", a, b);
}
