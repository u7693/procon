use procon::prelude::*;

fn main() {
    input! {
        n: isize,
        mut x: isize,
        mut y: isize,
    }
    x -= 1;
    y -= 1;
    let mut result = vec![0; n as usize];
    for i in 0..n {
        for j in i..n {
            let a = (j - i).abs();
            let b = (x - i).abs() + 1 + (j - y).abs();
            let c = (y - i).abs() + 1 + (j - x).abs();

            result[a.min(b).min(c) as usize] += 1;
        }
    }
    for i in 1..n {
        println!("{}", result[i as usize]);
    }
}
