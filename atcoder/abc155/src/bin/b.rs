use procon::prelude::*;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    for x in a {
        if x % 2 == 0 && !(x % 3 == 0 || x % 5 == 0) {
            println!("DENIED");
            return;
        }
    }

    println!("APPROVED");
}
