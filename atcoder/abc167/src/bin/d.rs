use proconio::input;

struct Solve {
    n: usize,
    k: u64,
    a: Vec<usize>,
}

impl Solve {
    fn new() -> Self {
        input! {
            n: usize, k: u64,
            mut a: [usize; n],
        }
        for x in &mut a {
            *x -= 1;
        }
        Self { n, k, a }
    }

    fn cycle(&self) -> (u64, u64) {
        let mut check: Vec<isize> = vec![-1; self.n];
        let mut idx = 0;
        check[idx] = 0;

        while check[self.a[idx]] == -1 {
            check[self.a[idx]] = check[idx] + 1;
            idx = self.a[idx];
        }
        // dbg!(&check);

        let x = check[self.a[idx]];
        let y = check[idx] - check[self.a[idx]] + 1;
        (x as u64, y as u64)
    }

    fn restore(&self, a: u64, b: u64) -> usize {
        let mut counter = a + b;
        let mut idx = 0;
        while counter > 0 {
            idx = self.a[idx];
            counter -= 1;
        }
        idx
    }

    fn solve(&self) {
        let (x, y) = self.cycle();
        // dbg!(x, y);
        let (a, b) = if x >= self.k {
            (self.k, 0)
        } else {
            (x, (self.k - x) % y)
        };
        // dbg!(a, b);
        println!("{}", self.restore(a, b) + 1);
    }
}

fn main() {
    Solve::new().solve();
}
