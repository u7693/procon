use proconio::input;

fn main() {
    input! {
        k: isize, x: isize,
    }
    for i in (x-k+1)..(k+x) {
        if i != x-k+1 {
            print!(" ");
        }
        print!("{}", i);
    }
    println!("");
}
