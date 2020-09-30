use cargo_snippet::snippet;

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
