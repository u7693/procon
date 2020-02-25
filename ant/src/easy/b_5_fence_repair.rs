pub fn solve(_n: usize, l: Vec<usize>) -> usize {
    use std::cmp::Reverse;
    use std::collections::BinaryHeap;

    let l: Vec<Reverse<usize>> = l.into_iter().map(|x| Reverse(x)).collect();
    let mut heap = BinaryHeap::from(l);
    let mut res = 0;

    while heap.len() > 1 {
        let s = heap.pop().unwrap().0 + heap.pop().unwrap().0;
        res += s;
        heap.push(Reverse(s));
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let l = vec![8, 5, 8];
        assert_eq!(solve(n, l), 34);
    }
}
