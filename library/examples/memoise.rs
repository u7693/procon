use procon::prelude::*;

#[memoise(keys(n = 100, m = 50))]
fn comb(n: usize, m: usize) -> usize {
    if m == 0 {
        return 1;
    }
    if n == 0 {
        return 0;
    }
    comb(n - 1, m - 1) + comb(n - 1, m)
}

fn main() {
    let _a = comb(10, 5);   // calculation
    comb_reset();           // reset the memoization table
    let a = comb(10, 5);    // calculation executed again

    println!("comb(10, 5) = {}", a);
}
