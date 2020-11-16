use proconio::input;

fn main() {
    input! { sx: f64, sy: f64 }
    input! { gx: f64, mut gy: f64 }

    gy *= -1.0;

    let a = (gy-sy) / (gx-sx);
    let b = sy - a * sx;

    println!("{:?}", (0.0 - b) / a);
}
