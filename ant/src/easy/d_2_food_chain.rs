pub struct UnionFind {
    par: Vec<usize>,
    rank: Vec<usize>,
}

impl UnionFind {
    pub fn init(n: usize) -> Self {
        let mut par = Vec::with_capacity(n);
        for i in 0..n {
            par.push(i);
        }
        let mut rank = Vec::with_capacity(n);
        for _i in 0..n {
            rank.push(0);
        }

        Self { par, rank }
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            x
        } else {
            self.par[x] = self.find(self.par[x]);
            self.par[x]
        }
    }

    pub fn unite(&mut self, x: usize, y: usize) {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return;
        }

        if self.rank[x] < self.rank[y] {
            self.par[x] = y;
        } else {
            self.par[y] = x;
            if self.rank[x] == self.rank[y] {
                self.rank[x] += 1;
            }
        }
    }

    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}

pub fn solve(n: usize, k: usize, vt: Vec<usize>, vx: Vec<usize>, vy: Vec<usize>) -> usize {
    let mut uf = UnionFind::init(n * 3);
    let mut ans = 0;

    for i in 0..k {
        let t = vt[i];
        let x = vx[i] as isize - 1;
        let y = vy[i] as isize - 1;

        if x < 0 || n as isize <= x || y < 0 || n as isize <= y {
            ans += 1;
            continue;
        }

        let x = x as usize;
        let y = y as usize;

        if t == 1 {
            if uf.same(x, y + n) || uf.same(x, y + 2 * n) {
                ans += 1;
            } else {
                uf.unite(x, y);
                uf.unite(x + n, y + n);
                uf.unite(x + 2 * n, y + 2 * n);
            }
        } else {
            if uf.same(x, y) || uf.same(x, y + 2 * n) {
                ans += 1;
            } else {
                uf.unite(x, y + n);
                uf.unite(x + n, y + 2 * n);
                uf.unite(x + 2 * n, y);
            }
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 100;
        let k = 7;
        let t = vec![1, 2, 2, 2, 1, 2, 1];
        let x = vec![101, 1, 2, 3, 1, 3, 5];
        let y = vec![1, 2, 3, 3, 3, 1, 5];

        assert_eq!(solve(n, k, t, x, y), 3);
    }
}
