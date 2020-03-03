const MAX_N: usize = 1000;
const MAX_M: usize = 1000;

pub fn solve(n: isize, m: isize, modulo: isize) -> isize {
    let mut dp: Vec<Vec<isize>> = vec![vec![0; MAX_N + 1]; MAX_M + 1];
    dp[0][0] = 1;

    for i in 1..=m {
        for j in 0..=n {
            dp[i as usize][j as usize] = if j - i >= 0 {
                (dp[i as usize - 1][j as usize] + dp[i as usize][(j - i) as usize]) % modulo
            } else {
                dp[i as usize - 1][j as usize]
            };
        }
    }

    dp[m as usize][n as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let m = 3;
        let modulo = 10000;
        assert_eq!(solve(n, m, modulo), 4);
    }
}
