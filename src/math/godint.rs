use cargo_snippet::snippet;
use std::ops::{Add, Sub, Mul, Div};
use std::ops::{AddAssign, SubAssign, MulAssign, DivAssign};

#[snippet]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct GodInt<const N: i64>(i64);

#[snippet("GodInt_Add")]
#[snippet(prefix = "use std::ops::Add;")]
impl<const N: i64> Add for GodInt<N> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut result = self;
        result += other;
        result
    }
}

#[snippet("GodInt_Add")]
#[snippet(prefix = "use std::ops::AddAssign;")]
impl<const N: i64> AddAssign for GodInt<N> {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        if self.0 >= N {
            self.0 -= N;
        }
    }
}

#[snippet("GodInt_Sub")]
#[snippet(prefix = "use std::ops::Sub;")]
impl<const N: i64> Sub for GodInt<N> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        let mut result = self;
        result -= other;
        result
    }
}

#[snippet("GodInt_Sub")]
#[snippet(prefix = "use std::ops::SubAssign;")]
impl<const N: i64> SubAssign for GodInt<N> {
    fn sub_assign(&mut self, other: Self) {
        if self.0 < other.0 {
            self.0 += N;
        }
        self.0 -= other.0;
    }
}

#[snippet("GodInt_Mul")]
#[snippet(prefix = "use std::ops::Mul;")]
impl<const N: i64> Mul for GodInt<N> {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        let mut result = self;
        result *= other;
        result
    }
}

#[snippet("GodInt_Mul")]
#[snippet(prefix = "use std::ops::MulAssign;")]
impl<const N: i64> MulAssign for GodInt<N> {
    fn mul_assign(&mut self, other: Self) {
        self.0  = (self.0 * other.0) % N;
    }
}

#[snippet("GodInt_Div")]
#[snippet(prefix = "use std::ops::Div;")]
impl<const N: i64> Div for GodInt<N> {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        let mut result = self;
        result /= other;
        result
    }
}

#[snippet("GodInt_Div")]
#[snippet(prefix = "use std::ops::DivAssign;")]
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

#[snippet("GodInt_Pow")]
impl<const N: i64> GodInt<N> {
    pub fn pow(self, mut exp: u32) -> Self {
        let mut base = self;
        let mut acc = GodInt::<N>(1);
        while exp > 1 {
            if (exp & 1) == 1 {
                acc = acc * base;
            }
            exp /= 2;
            base = base * base;
        }
        if exp == 1 {
            acc = acc * base;
        }
        acc
    }
}
