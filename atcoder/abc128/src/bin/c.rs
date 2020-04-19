use proconio::input;
use fixedbitset::FixedBitSet;

struct Solve {
    n: usize,
    m: usize,
    s: Vec<Vec<usize>>,
    p: Vec<usize>,
}

impl Solve {
    fn new() -> Self {
        input! { n: usize, m: usize };
        let mut s = Vec::new();
        for _ in 0..m {
            input! { k: usize, sv: [usize; k], };
            s.push(sv);
        }
        input! { p: [usize; m] };
        Self { n, m, s, p }
    }

    fn check(&self, bits: FixedBitSet) -> bool {
        for (idx, a) in self.s.iter().enumerate() {
            let mut counter = 0;
            for b in a {
                if bits[*b - 1] {
                    counter += 1;
                }
            }
            if counter % 2 != self.p[idx] {
                return false;
            }
        }
        return true;
    }

    fn solve(&self) -> usize {
        let mut result = 0;
        for i in 0..(1 << self.n) {
            let mut bits = FixedBitSet::with_capacity(self.n);
            for j in 0..self.n {
                bits.set(j, i & (1 << j) > 0);
            }
            dbg!("{}", &bits);
            if self.check(bits) {
                result += 1;
            }
        }
        return result;
    }
}

fn main() {
    println!("{}", Solve::new().solve());
}
