fn dfs(x: isize, y: isize, n: &isize, m: &isize, field: &mut Vec<Vec<char>>) {
    field[x as usize][y as usize] = '.';

    for dx in -1..2 {
        for dy in -1..2 {
            let nx = x + dx;
            let ny = y + dy;

            if 0 <= nx && nx < *n && 0 <= ny && ny < *m && field[nx as usize][ny as usize] == 'W' {
                dfs(nx, ny, n, m, field);
            }
        }
    }
}

pub fn solve(n: &isize, m: &isize, field: &mut Vec<Vec<char>>) -> isize {
    let mut res = 0;
    for i in 0..*n {
        for j in 0..*m {
            if field[i as usize][j as usize] == 'W' {
                dfs(i, j, n, m, field);
                res += 1;
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let n = 10;
        let m = 12;
        let mut field = vec![
            "W........WW.".chars().collect(),
            ".WWW.....WWW".chars().collect(),
            "....WW...WW.".chars().collect(),
            ".........WW.".chars().collect(),
            ".........W..".chars().collect(),
            "..W......W..".chars().collect(),
            ".W.W.....WW.".chars().collect(),
            "W.W.W.....W.".chars().collect(),
            ".W.W......W.".chars().collect(),
            "..W.......W.".chars().collect(),
        ];
        assert_eq!(solve(&n, &m, &mut field), 3)
    }
}
