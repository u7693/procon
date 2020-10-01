#![feature(min_const_generics)]

use proconio::input;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GodInt<const N: i64>(i64);
use std::ops::Add;
use std::ops::AddAssign;
impl<const N: i64> Add for GodInt<N> {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        let mut result = self;
        result += other;
        result
    }
}
impl<const N: i64> AddAssign for GodInt<N> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        if self.0 >= N {
            self.0 -= N;
        }
    }
}

const MOD: i64 = 1000_000_000 + 7;

fn main() {
    input! { s: usize }
    let mut dp = vec![GodInt::<MOD>(0); 2100];
    dp[0] = GodInt::<MOD>(1);
    for i in 3..=s {
        dp[i] = dp[i-1] + dp[i-3];
    }
    println!("{}", dp[s].0);
}
