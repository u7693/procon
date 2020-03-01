use std::cmp;

const MAX_N: usize = 100;
const MAX_W: usize = 10000;

pub struct Solver1 {
    n: usize,
    weight: Vec<usize>,
    value: Vec<usize>,
    dp: Vec<Vec<isize>>,
}

impl Solver1 {
    fn rec(&mut self, i: usize, j: usize) -> usize {
        if self.dp[i][j] >= 0 {
            return self.dp[i][j] as usize;
        }

        let res = if i == self.n {
            0
        } else if j < self.weight[i] {
            self.rec(i + 1, j)
        } else {
            cmp::max(
                self.rec(i + 1, j),
                self.rec(i + 1, j - self.weight[i]) + self.value[i],
            )
        };

        self.dp[i][j] = res as isize;
        res
    }

    pub fn solve(n: usize, weight: Vec<usize>, value: Vec<usize>, w: usize) -> usize {
        let dp = vec![vec![-1; MAX_N + 1]; MAX_W + 1];
        let mut solver = Self {
            n,
            weight,
            value,
            dp,
        };
        solver.rec(0, w)
    }
}

pub struct Solver2;

impl Solver2 {
    pub fn solve(n: usize, weight: Vec<usize>, value: Vec<usize>, w: usize) -> usize {
        let mut dp = vec![vec![0; MAX_N + 1]; MAX_W + 1];
        for i in (0..n).rev() {
            for j in 0..w + 1 {
                dp[i][j] = if j < weight[i] {
                    dp[i + 1][j]
                } else {
                    cmp::max(dp[i + 1][j], dp[i + 1][j - weight[i]] + value[i])
                };
            }
        }
        dp[0][w]
    }
}

pub struct Solver3;

impl Solver3 {
    pub fn solve(n: usize, weight: Vec<usize>, value: Vec<usize>, w: usize) -> usize {
        let mut dp = vec![vec![0; MAX_N + 1]; MAX_W + 1];
        for i in 0..n {
            for j in 0..w + 1 {
                dp[i + 1][j] = if j < weight[i] {
                    dp[i][j]
                } else {
                    cmp::max(dp[i][j], dp[i][j - weight[i]] + value[i])
                };
            }
        }
        dp[n][w]
    }
}

pub struct Solver4;

impl Solver4 {
    pub fn solve(n: usize, weight: Vec<usize>, value: Vec<usize>, w: usize) -> usize {
        let mut dp = vec![vec![0; MAX_N + 1]; MAX_W + 1];
        for i in 0..n {
            for j in 0..w + 1 {
                dp[i + 1][j] = cmp::max(dp[i + 1][j], dp[i][j]);
                if j + weight[i] <= w {
                    dp[i + 1][j + weight[i]] =
                        cmp::max(dp[i + 1][j + weight[i]], dp[i][j] + value[i]);
                }
            }
        }
        dp[n][w]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let weight = vec![2, 1, 3, 2];
        let value = vec![3, 2, 4, 2];
        let w = 5;
        assert_eq!(Solver1::solve(n, weight, value, w), 7);
    }

    #[test]
    fn case2() {
        let n = 4;
        let weight = vec![2, 1, 3, 2];
        let value = vec![3, 2, 4, 2];
        let w = 5;
        assert_eq!(Solver2::solve(n, weight, value, w), 7);
    }

    #[test]
    fn case3() {
        let n = 4;
        let weight = vec![2, 1, 3, 2];
        let value = vec![3, 2, 4, 2];
        let w = 5;
        assert_eq!(Solver3::solve(n, weight, value, w), 7);
    }

    #[test]
    fn case4() {
        let n = 4;
        let weight = vec![2, 1, 3, 2];
        let value = vec![3, 2, 4, 2];
        let w = 5;
        assert_eq!(Solver4::solve(n, weight, value, w), 7);
    }
}
