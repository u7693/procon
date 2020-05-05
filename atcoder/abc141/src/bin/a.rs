use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let r = match &*s {
        "Sunny" => "Cloudy",
        "Cloudy" => "Rainy",
        "Rainy" => "Sunny",
        _ => unreachable!(),
    };

    println!("{}", r);
}
