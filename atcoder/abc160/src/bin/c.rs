use procon::prelude::*;
use std::collections::BinaryHeap;

fn main() {
    input!{
        k: usize,
        n: usize,
        a: [usize; n],
    }
    let mut heap = BinaryHeap::new();
    heap.push(
        k - a[n-1] + a[0]
    );
    for i in 0..n-1 {
        heap.push(a[i+1]-a[i]);
    }
    let x = heap.peek().unwrap();
    println!("{}", k - x);
}
