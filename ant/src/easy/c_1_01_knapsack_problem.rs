use std::cmp;

const MAX_N: usize = 100;
const MAX_W: usize = 10000;

pub struct Solver {
    n: usize,
    weight: Vec<usize>,
    value: Vec<usize>,
    dp: Vec<Vec<isize>>,
}

impl Solver {
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
        return res;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let weight = vec![2, 1, 3, 2];
        let value = vec![3, 2, 4, 2];
        let w = 5;
        assert_eq!(Solver::solve(n, weight, value, w), 7);
    }
}
