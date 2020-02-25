const V: [usize; 6] = [1, 5, 10, 50, 100, 500];

pub fn solve(c: &[usize], mut a: usize) -> usize {
    let mut ans = 0;
    for i in (0..6).rev() {
        let t = c[i].min(a / V[i]);
        a -= t * V[i];
        ans += t;
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let c = [3, 2, 1, 3, 0, 2];
        let a = 620;
        assert_eq!(solve(&c, a), 6)
    }
}
