pub struct Solver {
    color: Vec<isize>,
    graph: Vec<Vec<usize>>,
}

impl Solver {
    pub fn dfs(&mut self, v: usize, c: isize) -> bool {
        self.color[v] = c;
        for i in 0..self.graph[v].len() {
            if self.color[self.graph[v][i]] == c {
                return false;
            }
            if self.color[self.graph[v][i]] == 0 && !self.dfs(self.graph[v][i], -c) {
                return false;
            }
        }
        true
    }

    pub fn solve(n: usize, graph: Vec<Vec<usize>>) -> bool {
        let mut solver = Solver {
            color: vec![0; n],
            graph,
        };
        for i in 0..n {
            if solver.color[i] == 0 && !solver.dfs(i, 1) {
                return false;
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 3;
        let g = vec![vec![1, 2], vec![0, 2], vec![0, 1]];

        assert_eq!(Solver::solve(n, g), false);
    }

    #[test]
    fn case2() {
        let n = 4;
        let g = vec![vec![1, 3], vec![0, 2], vec![1, 3], vec![0, 2]];

        assert_eq!(Solver::solve(n, g), true);
    }
}
