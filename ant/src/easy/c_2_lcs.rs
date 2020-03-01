use std::cmp;

const MAX_N: usize = 1000;
const MAX_M: usize = 1000;

pub fn solve(n: usize, m: usize, s: String, t: String) -> usize {
    let mut dp = vec![vec![0; MAX_M + 1]; MAX_N + 1];
    let s: Vec<char> = s.chars().collect();
    let t: Vec<char> = t.chars().collect();

    assert_eq!(s.len(), n);
    assert_eq!(t.len(), m);

    for i in 0..n {
        for j in 0..m {
            dp[i + 1][j + 1] = if s[i] == t[j] {
                dp[i][j] + 1
            } else {
                cmp::max(dp[i][j + 1], dp[i + 1][j])
            };
        }
    }

    dp[n][m]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let m = 4;
        let s = "abcd".to_string();
        let t = "becd".to_string();
        assert_eq!(solve(n, m, s, t), 3);
    }
}
