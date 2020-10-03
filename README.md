# procon

## tips

### `String`と`Vec<char>`の変換

```rust
fn string2char(s: &String) -> Vec<char> {
    s.chars().collect::<Vec<char>>()
}

fn char2string(c: &Vec<char>) -> String {
    c.iter().collect::<String>()
}

let s = "Hello".to_string();
assert_eq!(s, char2string(&string2char(&s)));
```

### `Vec`のソート

```rust
use std::cmp::Reverse;

let mut v = vec![5, 1, 3, 4, 2];

// 昇順でソート
v.sort();
assert_eq!(v, vec![1, 2, 3, 4, 5]);

// 降順でソート
v.sort_by_key(|&k| Reverse(k));
assert_eq!(v, vec![5, 4, 3, 2, 1]);
```

### `Map`のインクリメント

```rust
use std::cmp::Ord;
use std::collections::BTreeMap;

fn insert_or_increment<K: Ord>(map: &mut BTreeMap<K, usize>, key: K) {
    map.entry(key).and_modify(|e| *e += 1).or_insert(1);
}

let mut map = BTreeMap::new();

insert_or_increment(&mut map, 'a');
insert_or_increment(&mut map, 'a');
insert_or_increment(&mut map, 'a');
insert_or_increment(&mut map, 'b');

assert_eq!(map[&'a'], 3);
assert_eq!(map[&'b'], 1);
```
