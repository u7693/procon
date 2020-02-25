pub fn solve(n: usize, r: usize, x: &mut Vec<usize>) -> usize {
    x.sort();

    let mut i = 0;
    let mut ans = 0;

    while i < n {
        let s = x[i];
        while i < n && x[i] <= s + r {
            i += 1;
        }

        let p = x[i - 1];
        while i < n && x[i] <= p + r {
            i += 1;
        }

        ans += 1;
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 6;
        let r = 10;
        let mut x = vec![1, 7, 15, 20, 30, 50];
        assert_eq!(solve(n, r, &mut x), 3);
    }
}
