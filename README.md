# procon

## tips

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
