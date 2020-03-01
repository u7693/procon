const MAX_K: usize = 100000;

pub fn solve(n: usize, a: Vec<usize>, m: Vec<usize>, k: usize) -> bool {
    let mut dp: Vec<isize> = vec![-1; MAX_K + 1];
    dp[0] = 0;

    for i in 0..n {
        for j in 0..k + 1 {
            dp[j] = if dp[j] >= 0 {
                m[i] as isize
            } else if j < a[i] || dp[j - a[i]] <= 0 {
                -1
            } else {
                dp[j - a[i]] - 1
            };
        }
    }

    dp[k] >= 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let a = vec![3, 5, 8];
        let m = vec![3, 2, 2];
        let k = 17;
        assert_eq!(solve(n, a, m, k), true);
    }
}
