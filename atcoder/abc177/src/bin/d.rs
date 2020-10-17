use proconio::input;
use petgraph::unionfind::UnionFind;
use std::collections::BTreeMap;
use std::cmp::Ord;

fn insert_or_increment<K: Ord>(map: &mut BTreeMap<K, usize>, key: K) {
    map.entry(key).and_modify(|e| *e += 1).or_insert(1);
}

fn main() {
    input! { n: usize, m: usize }
    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        input! { a: usize, b: usize }
        uf.union(a-1, b-1);
    }
    let mut map = BTreeMap::new();
    for i in uf.into_labeling() {
        insert_or_increment(&mut map, i);
    }
    let mut result = 0;
    for (_, v) in map.iter() {
        result = result.max(*v);
    }
    println!("{}", result);
}
