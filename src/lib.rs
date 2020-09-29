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

#[snippet]
pub fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet]
#[snippet(include = "gcd")]
pub fn lcm(a: u64, b: u64) -> u64 {
    a / gcd(a, b) * b
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

    #[test]
    fn test_gcd() {
        let testcases = vec![(11, 121, 11), (18, 27, 9), (21, 56, 7)];
        for testcase in testcases {
            assert_eq!(gcd(testcase.0, testcase.1), testcase.2);
        }
    }

    #[test]
    fn test_lcm() {
        let testcases = vec![(0, 1, 0), (18, 27, 54), (49, 77, 539)];
        for testcase in testcases {
            assert_eq!(lcm(testcase.0, testcase.1), testcase.2);
        }
    }
}
