use procon::prelude::*;
use std::collections::BTreeMap;

fn main() {
    input!{
        n: usize,
        s: [String; n],
    }

    let mut result = BTreeMap::new();
    for x in s {
        if let Some(v) = result.get_mut(&x) {
            *v += 1;
        } else {
            result.insert(x, 1);
        }
    }
    let mut maxv = 0;
    for (k, v) in &result {
        maxv = maxv.max(*v);
    }
    for (k, v) in &result {
        if *v == maxv {
            println!("{}", k);
        }
    }
}
