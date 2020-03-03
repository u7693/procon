pub fn solve(n: isize, _l: isize, p: isize, a: Vec<isize>, b: Vec<isize>) -> isize {
    use std::collections::BinaryHeap;

    let mut que = BinaryHeap::new();
    let mut ans = 0;
    let mut pos = 0;
    let mut tank = p;

    for i in 0..n {
        let d = a[i as usize] - pos;

        while tank - d < 0 {
            if que.is_empty() {
                return -1;
            }

            tank += que.pop().unwrap();
            ans += 1;
        }

        tank -= d;
        pos = a[i as usize];
        que.push(b[i as usize]);
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let mut n = 4;
        let l = 25;
        let p = 10;
        let mut a = vec![10, 14, 20, 21];
        let mut b = vec![10, 5, 2, 4];

        a.push(l);
        b.push(0);
        n += 1;

        assert_eq!(solve(n, l, p, a, b), 2);
    }
}
