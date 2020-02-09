use procon::prelude::*;

fn main() {
    input! {
        mut h: isize,
        a: isize,
    }

    let mut ans = 0;
    while h > 0 {
        h -= a;
        ans += 1;
    }

    println!("{}", ans);
}
