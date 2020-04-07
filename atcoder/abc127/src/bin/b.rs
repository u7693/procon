use procon::prelude::*;

fn main() {
    input! {
        r: usize,
        d: usize,
        mut x: usize,
    }

    for i in 0..10 {
        x = r * x - d;
        println!("{}", x);
    }
}
