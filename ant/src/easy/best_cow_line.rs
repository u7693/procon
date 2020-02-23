pub fn solve(_n: usize, s: &str) -> String {
    use std::collections::VecDeque;

    let mut t = String::new();
    let mut a = VecDeque::from(s.chars().collect::<Vec<char>>());
    let mut b = VecDeque::from(s.chars().rev().collect::<Vec<char>>());

    while a.len() != 0 {
        if a < b {
            t.push(a.pop_front().unwrap());
            b.pop_back();
        } else {
            t.push(b.pop_front().unwrap());
            a.pop_back();
        }
    }

    t.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 6;
        let s = "ACDBCB";
        assert_eq!(solve(n, &s), "ABCBCD".to_string());
    }
}
