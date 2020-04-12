use procon::prelude::*;
use procon::proconio::marker::Chars;

struct Solve {
    r: Vec<isize>,
    g: Vec<isize>,
    b: Vec<isize>,
}

impl Solve {
    fn new() -> Self {
        input! {
            n: usize,
            s: Chars,
        }

        let mut r = Vec::new();
        let mut g = Vec::new();
        let mut b = Vec::new();

        for i in 0..n {
            match s[i] {
                'R' => r.push(i as isize),
                'G' => g.push(i as isize),
                'B' => b.push(i as isize),
                _ => unreachable!(),
            }
        }

        Self { r, g, b }
    }

    fn solve(&self) -> usize {
        let mut result = 0;

        for r in self.r.iter() {
            for g in self.g.iter() {
                result += self.search_b(*r, *g);
            }
        }

        result
    }

    fn search_b(&self, r: isize, g: isize) -> usize {
        let mut n = self.b.len();
        let k = r - g;

        let x = r + k;
        n -= self.b.upper_bound(&x) - self.b.lower_bound(&x);

        let x = g - k;
        n -= self.b.upper_bound(&x) - self.b.lower_bound(&x);

        if k % 2 == 0 {
            let x = r - k / 2;
            n -= self.b.upper_bound(&x) - self.b.lower_bound(&x);
        }

        n
    }
}

fn main() {
    println!("{}", Solve::new().solve());
}
