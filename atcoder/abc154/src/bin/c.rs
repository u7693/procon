use procon::prelude::*;

fn main() {
    input!{
        n: usize,
        mut a: [usize; n],
    }
    a.sort();

    for i in 0..n-1 {
        if a[i] == a[i+1] {
            println!("NO");
            return;
        }
    }

    println!("YES");
}
