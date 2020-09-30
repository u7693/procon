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
use std::ops::Sub;
use std::ops::SubAssign;
impl<const N: i64> Sub for GodInt<N> {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}
impl<const N: i64> SubAssign for GodInt<N> {
    fn sub_assign(&mut self, other: Self) {
        if self.0 < other.0 {
            self.0 += N;
        }
        self.0 -= other.0;
    }
}
use std::ops::Div;
use std::ops::DivAssign;
impl<const N: i64> Div for GodInt<N> {
    type Output = Self;
    fn div(self, other: Self) -> Self {
        let mut result = self;
        result /= other;
        result
    }
}
impl<const N: i64> DivAssign for GodInt<N> {
    fn div_assign(&mut self, mut other: Self) {
        let mut exp = N - 2i64;
        while exp != 0 {
            if exp % 2 != 0 {
                self.0 *= other.0;
            }
            other.0 *= other.0;
            exp /= 2;
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
    input! { n: i64 }
    let (n10, n9, n8) = {
        let mut tmp10 = GodInt::<MOD>(1);
        let mut tmp9  = GodInt::<MOD>(1);
        let mut tmp8  = GodInt::<MOD>(1);
        for _ in 0..n {
            tmp10 *= GodInt::<MOD>(10);
            tmp9  *= GodInt::<MOD>(9 );
            tmp8  *= GodInt::<MOD>(8 );
        }
        (tmp10, tmp9, tmp8)
    };
    println!("{}", (n10 - n9 - n9 + n8).0);
}
