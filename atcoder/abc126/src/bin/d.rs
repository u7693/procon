use procon::prelude::*;

#[derive(Clone)]
struct Edge {
    to: usize,
    dist: usize,
}

struct Solve {
    n: usize,
    g: Vec<Vec<Edge>>,
    v: Vec<i8>,
    check: Vec<bool>,
    start: usize,
}

impl Solve {
    fn new() -> Self {
        input! {
            n: usize,
            uvw: [[usize; 3]; n-1],
        }

        let mut g = vec![Vec::new(); n];
        let mut v = vec![0; n];
        let mut check = vec![false; n];

        let mut start = 0;

        for x in uvw {
            let u = x[0] - 1;
            let v = x[1] - 1;
            let w = x[2];

            g[u].push(Edge { to: v, dist: w });
            g[v].push(Edge { to: u, dist: w });

            start = u;
        }

        Self { n, g, v, check, start }
    }

    fn dfs(&mut self, parent: usize, color: i8) {
        self.v[parent] = color;
        self.check[parent] = true;

        for e in self.g[parent].clone() {
            if self.check[e.to] {
                continue;
            }

            let c = if e.dist % 2 == 1 {
                color * -1
            } else {
                color
            };

            self.dfs(e.to, c);
        }
    }

    fn solve(&mut self) {
        self.dfs(self.start, 1);
        for x in self.v.clone() {
            if x == -1 {
                println!("{}", 0);
            } else {
                println!("{}", 1);
            }
        }
    }
}

fn main() {
    Solve::new().solve();
}
