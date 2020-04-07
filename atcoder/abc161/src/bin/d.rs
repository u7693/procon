use std::collections::VecDeque;
use procon::prelude::*;

fn main() {
    input! {
        mut k: isize,
    }
    let mut v: VecDeque<i64> = VecDeque::new();
    for i in 1..=9 {
        v.push_back(i);
    }
    while k - 1 > 0 {
        let x = v.pop_front().unwrap();
        k -= 1;

        if x % 10 != 0 {
            v.push_back(10 * x + x % 10 - 1);
        }
        v.push_back(10 * x + x % 10);
        if x % 10 != 9 {
            v.push_back(10 * x + x % 10 + 1);
        }
    }
    println!("{}", v.pop_front().unwrap());
}
