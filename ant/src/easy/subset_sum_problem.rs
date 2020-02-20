use procon::prelude::*;

pub fn dfs(i: usize, sum: usize, n: &usize, a: &[usize], k: &usize) -> bool {
    if i == *n {
        return sum == *k;
    }

    if dfs(i + 1, sum, n, a, k) {
        return true;
    }

    if dfs(i + 1, sum + a[i], n, a, k) {
        return true;
    }

    return false;
}

pub fn solve() {
    input! {
        n: usize,
        a: [usize; n],
        k: usize,
    }

    let result = if dfs(0, 0, &n, &a, &k) { "Yes" } else { "No" };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 4;
        let a = [1, 2, 4, 7];
        let k = 13;
        assert!(dfs(0, 0, &n, &a, &k));
    }

    #[test]
    fn case2() {
        let n = 4;
        let a = [1, 2, 4, 7];
        let k = 15;
        assert!(!dfs(0, 0, &n, &a, &k));
    }
}
