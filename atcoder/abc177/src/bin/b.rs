use proconio::input;

fn main() {
    input! { s: String, t: String }
    let s = s.chars().collect::<Vec<char>>();
    let t = t.chars().collect::<Vec<char>>();
    let mut result = 2000;
    for si in 0..=(s.len()-t.len()) {
        let mut diff = 0;
        for ti in 0..(t.len()) {
            if s[si+ti] != t[ti] {
                diff += 1;
            }
        }
        result = result.min(diff);
    }
    println!("{}", result);
}
