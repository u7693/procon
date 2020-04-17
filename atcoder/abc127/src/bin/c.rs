use procon::prelude::*;

fn main() {
    input!{
        n: usize,
        m: usize,
        lr: [[isize; 2]; m],
    }

    let mut max_l = 0;
    let mut min_r = 1000000;

    for x in lr {
        let l = x[0];
        let r = x[1];

        max_l = max_l.max(l);
        min_r = min_r.min(r);
    }

    let result = min_r - max_l + 1;

    if result < 0 {
        println!("{}", 0);
    } else {
        println!("{}", result);
    }
}
