use procon::prelude::*;

fn main() {
    input! {
        mut h: u64,
    }

    let mut count: u64 = 1;
    let mut ans: u64 = 0;

    while h > 1 {
        ans += count;
        count *= 2;
        h /= 2;
    }

    println!("{}", ans + count);
}
