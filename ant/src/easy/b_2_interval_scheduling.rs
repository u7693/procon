pub fn solve(n: usize, s: &Vec<usize>, t: &Vec<usize>) -> usize {
    let mut itv = t.iter().zip(s.iter()).collect::<Vec<_>>();
    itv.sort();

    let mut ans = 0;
    let mut t = 0;

    for i in 0..n {
        if t < *itv[i].1 {
            ans += 1;
            t = *itv[i].0
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 5;
        let s = vec![1, 2, 4, 6, 8];
        let t = vec![3, 5, 7, 9, 10];
        assert_eq!(solve(n, &s, &t), 3)
    }
}
