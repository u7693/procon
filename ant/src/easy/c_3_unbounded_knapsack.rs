use std::cmp;

const MAX_N: usize = 100;
const MAX_W: usize = 10000;

pub fn solve1(n: usize, weight: Vec<usize>, value: Vec<usize>, w: usize) -> usize {
    let mut dp = vec![vec![0; MAX_W + 1]; MAX_N + 1];
    for i in 0..n {
        for j in 0..w + 1 {
            dp[i + 1][j] = if j < weight[i] {
                dp[i][j]
            } else {
                cmp::max(dp[i][j], dp[i + 1][j - weight[i]] + value[i])
            };
        }
    }

    dp[n][w]
}

pub fn solve2(n: usize, weight: Vec<usize>, value: Vec<usize>, w: usize) -> usize {
    let mut dp = vec![0; MAX_W + 1];
    for i in 0..n {
        for j in weight[i]..w + 1 {
            dp[j] = cmp::max(dp[j], dp[j - weight[i]] + value[i]);
        }
    }

    dp[w]
}

pub fn solve3(n: usize, weight: Vec<usize>, value: Vec<usize>, w: usize) -> usize {
    let mut dp = vec![vec![0; MAX_W + 1]; 2];
    for i in 0..n {
        for j in 0..w + 1 {
            dp[(i + 1) & 1][j] = if j < weight[i] {
                dp[i & 1][j]
            } else {
                cmp::max(dp[i & 1][j], dp[(i + 1) & 1][j - weight[i]] + value[i])
            };
        }
    }

    dp[n & 1][w]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let weight = vec![3, 4, 2];
        let value = vec![4, 5, 3];
        let w = 7;
        assert_eq!(solve1(n, weight, value, w), 10);
    }

    #[test]
    fn case2() {
        let n = 3;
        let weight = vec![3, 4, 2];
        let value = vec![4, 5, 3];
        let w = 7;
        assert_eq!(solve2(n, weight, value, w), 10);
    }

    #[test]
    fn case3() {
        let n = 3;
        let weight = vec![3, 4, 2];
        let value = vec![4, 5, 3];
        let w = 7;
        assert_eq!(solve3(n, weight, value, w), 10);
    }
}
