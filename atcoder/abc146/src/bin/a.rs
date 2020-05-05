use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let d = vec!["SUN", "MON", "TUE", "WED", "THU", "FRI", "SAT"];
    for i in 0..7 {
        if s == d[i] {
            println!("{}", 7 - i);
            return;
        }
    }
}
