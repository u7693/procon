use proconio::input;
use fixedbitset::FixedBitSet;

struct Solve {
    n: usize,
    m: usize,
    x: usize,
    c: Vec<usize>,
    a: Vec<Vec<usize>>,
}

impl Solve {
    fn new() -> Self {
        input! {
            n: usize, m: usize, x: usize,
            ca: [[usize; m+1]; n],
        }

        let mut c = Vec::with_capacity(n);
        let mut a = Vec::with_capacity(n);

        for i in 0..n {
            c.push(ca[i][0]);
            a.push(ca[i][1..m+1].to_owned());
        }

        Self { n, m, x, c, a }
    }

    fn check(&self, flag: &FixedBitSet) -> isize {
        let mut check = vec![0; self.m];
        let mut cost = 0;
        for i in 0..self.n {
            if flag[i] {
                cost += self.c[i];
                for j in 0..self.m {
                    check[j] += self.a[i][j];
                }
            }
        }
        for x in check {
            if x < self.x {
                return -1;
            }
        }
        cost as isize
    }

    fn dfs(&self, mut flag: FixedBitSet, bit: usize, enabled: bool) -> isize {
        if bit == self.n {
            return self.check(&flag);
        }

        flag.set(bit, enabled);

        let a = self.dfs(flag.clone(), bit+1, true);
        let b = self.dfs(flag.clone(), bit+1, false);

        if a == -1 && b == -1 {
            -1
        } else if a == -1 {
            b
        } else if b == -1 {
            a
        } else {
            a.min(b)
        }
    }

    fn solve(&self) -> isize {
        let flag = FixedBitSet::with_capacity(self.n);
        let a = self.dfs(flag.clone(), 0, true);
        let b = self.dfs(flag.clone(), 0, false);

        if a == -1 && b == -1 {
            -1
        } else if a == -1 {
            b
        } else if b == -1 {
            a
        } else {
            a.min(b)
        }
    }
}

fn main() {
    println!("{}", Solve::new().solve());
}
