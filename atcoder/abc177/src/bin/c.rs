#![feature(min_const_generics)]

use proconio::input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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
use std::ops::Mul;
use std::ops::MulAssign;
impl<const N: i64> Mul for GodInt<N> {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        let mut result = self;
        result *= other;
        result
    }
}
impl<const N: i64> MulAssign for GodInt<N> {
    fn mul_assign(&mut self, other: Self) {
        self.0 = (self.0 * other.0) % N;
    }
}

const MOD: i64 = 1_000_000_000 + 7;

fn main() {
    input! { n: usize, a: [i64; n] }
    let b = {
        let mut b = Vec::with_capacity(n);
        let mut tmp = GodInt::<MOD>(0);
        for i in (0..n).rev() {
            tmp += GodInt::<MOD>(a[i]);
            b.push(tmp);
        }
        b.reverse();
        b
    };

    let mut result = GodInt::<MOD>(0);
    for i in 0..(n-1) {
        result += GodInt::<MOD>(a[i]) * b[i+1];
    }
    println!("{}", result.0);
}
