use procon::prelude::*;
use std::cmp;

const MAX_N: usize = 1000000;
const INF: usize = 10000000000;

pub fn solve1(n: usize, a: Vec<usize>) -> usize {
    let mut dp: Vec<usize> = vec![1; MAX_N + 1];
    let mut res = 0;

    for i in 0..n {
        for j in 0..i {
            if a[j] < a[i] {
                dp[i] = cmp::max(dp[i], dp[j] + 1);
            }
        }
        res = cmp::max(res, dp[i]);
    }

    res
}

pub fn solve2(n: usize, a: Vec<usize>) -> usize {
    let mut dp: Vec<usize> = vec![INF; MAX_N + 1];

    for i in 0..n {
        let t = dp.lower_bound(&a[i]);
        dp[t] = a[i];
    }

    dp.lower_bound(&INF)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 5;
        let a = vec![4, 2, 3, 1, 5];
        assert_eq!(solve1(n, a), 3);
    }

    #[test]
    fn case2() {
        let n = 5;
        let a = vec![4, 2, 3, 1, 5];
        assert_eq!(solve2(n, a), 3);
    }
}
