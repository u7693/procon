use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

struct Solve {
    n: usize,
    k: usize,
    v: Vec<isize>,
}

impl Solve {
    fn new() -> Self {
        input! { n: usize, k: usize, v: [isize; n] };
        Self { n, k, v }
    }

    fn solve(&self) -> isize {
        let mut result = -10000000 * 200;
        let r = self.n.min(self.k);

        for a in 0..=r {
            for b in 0..=(r - a) {
                let mut heap = BinaryHeap::new();
                for ai in 0..a {
                    heap.push(Reverse(self.v[ai]));
                }
                for bi in 0..b {
                    heap.push(Reverse(self.v[self.n - bi - 1]));
                }
                for _ in 0..(self.k - (a + b)) {
                    if let Some(x) = heap.peek() {
                        if x.0 < 0 {
                            heap.pop();
                        }
                    }
                }
                let mut vs = 0;
                for x in heap {
                    vs += x.0;
                }
                result = result.max(vs);
            }
        }

        result
    }
}

fn main() {
    println!("{}", Solve::new().solve());
}
