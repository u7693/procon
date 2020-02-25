const DX: [isize; 4] = [0, 0, -1, 1];
const DY: [isize; 4] = [-1, 1, 0, 0];
const INF: usize = 100000000;

type P = (usize, usize);

fn search_point(n: &usize, m: &usize, maze: &Vec<Vec<char>>, target: char) -> P {
    for i in 0..*n {
        for j in 0..*m {
            if maze[i][j] == target {
                return (i, j);
            }
        }
    }
    panic!("Not found")
}

fn bfs(n: &usize, m: &usize, maze: &Vec<Vec<char>>) -> usize {
    use std::collections::VecDeque;

    let mut queue: VecDeque<P> = VecDeque::new();
    let mut d = vec![vec![INF; *n]; *m];

    let s = search_point(n, m, maze, 'S');
    let g = search_point(n, m, maze, 'G');

    queue.push_back(s);
    d[s.0][s.1] = 0;

    while queue.len() != 0 {
        let point = queue.pop_front().unwrap();
        if point == g {
            break;
        }
        for i in 0..4 {
            let nx = (point.0 as isize) + DX[i];
            let ny = (point.1 as isize) + DY[i];

            if 0 <= nx && nx < (*n as isize) && 0 <= ny && ny < (*m as isize) {
                let nx = nx as usize;
                let ny = ny as usize;
                if maze[nx][ny] != '#' && d[nx][ny] == INF {
                    queue.push_back((nx, ny));
                    d[nx][ny] = d[point.0][point.1] + 1;
                }
            }
        }
    }

    d[g.0][g.1]
}

pub fn solve(n: &usize, m: &usize, maze: &Vec<Vec<char>>) -> usize {
    bfs(n, m, maze)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 10;
        let m = 10;
        let mut maze = vec![
            "#S######.#".chars().collect(),
            "......#..#".chars().collect(),
            ".#.##.##.#".chars().collect(),
            ".#........".chars().collect(),
            "##.##.####".chars().collect(),
            "....#....#".chars().collect(),
            ".#######.#".chars().collect(),
            "....#.....".chars().collect(),
            ".####.###.".chars().collect(),
            "....#...G#".chars().collect(),
        ];
        assert_eq!(solve(&n, &m, &mut maze), 22)
    }
}
