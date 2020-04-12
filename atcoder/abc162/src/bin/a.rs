use procon::prelude::*;

fn main() {
    input! {
        n: String,
    }

    let n: Vec<char> = n.chars().collect();

    for i in 0..3 {
        if n[i] == '7' {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
