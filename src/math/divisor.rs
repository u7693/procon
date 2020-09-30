use cargo_snippet::snippet;

#[snippet]
pub fn divisor(n: u64) -> Vec<u64> {
    let mut r = Vec::new();
    let mut i = 1u64;
    while i * i <= n {
        if n % i == 0 {
            r.push(i);
            if i * i != n {
                r.push(n / i);
            }
        }
        i += 1;
    }
    r.sort();
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divisor() {
        let testcases = vec![(3, vec![1, 3]), (6, vec![1, 2, 3, 6]), (25, vec![1, 5, 25])];
        for testcase in testcases {
            assert_eq!(divisor(testcase.0), testcase.1);
        }
    }
}
