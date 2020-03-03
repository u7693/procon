const MAX_N: usize = 1000;
const MAX_M: usize = 1000;

pub fn solve(n: isize, m: isize, a: Vec<isize>, modulo: isize) -> isize {
    let mut dp: Vec<Vec<isize>> = vec![vec![0; MAX_N + 1]; MAX_M + 1];

    for i in 0..=n {
        dp[i as usize][0] = 1;
    }

    for i in 0..n {
        for j in 1..=m {
            dp[(i + 1) as usize][j as usize] = if j - 1 - a[i as usize] >= 0 {
                (dp[(i + 1) as usize][(j - 1) as usize] + dp[i as usize][j as usize]
                    - dp[i as usize][(j - 1 - a[i as usize]) as usize]
                    + modulo)
                    % modulo
            } else {
                (dp[(i + 1) as usize][(j - 1) as usize] + dp[i as usize][j as usize]) % modulo
            };
        }
    }

    dp[(n) as usize][(m) as usize]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let m = 3;
        let a = vec![1, 2, 3];
        let modulo = 10000;
        assert_eq!(solve(n, m, a, modulo), 6);
    }
}
