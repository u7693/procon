# スニペット

## 文字列

```rust
# fn main() {
let s = "Hello".to_string();

// String -> Vec<char>
let c = s.chars().collect::<Vec<char>>();
assert_eq!(c, vec!['H', 'e', 'l', 'l', 'o']);

// n回繰り返す
let r = s.repeat(3);
assert_eq!(r, "HelloHelloHello".to_string());
# }
```

## 配列

```rust
# fn main() {
use std::cmp::Reverse;

let mut v = vec![5, 1, 3, 4, 2];

// 昇順でソート
v.sort();
assert_eq!(v, vec![1, 2, 3, 4, 5]);

// 降順でソート
v.sort_by_key(|&k| Reverse(k));
assert_eq!(v, vec![5, 4, 3, 2, 1]);
# }
```

```rust
# fn main() {
use std::cmp::Reverse;

let mut v = vec![
    (1, 3),
    (1, 4),
    (2, 5),
    (2, 6),
];

// 1列目は降順、2列目は昇順
v.sort_by_key(|k| (Reverse(k.0), k.1));
assert_eq!(v, vec![(2, 5), (2, 6), (1, 3), (1, 4)]);
# }
```

## 多次元配列

```rust
# fn main() {
// todo!();
# }
```

## グラフ

```rust
# fn main() {
// todo!();
# }
```

## 二分探索

```Rust
use superslice::*;

let a = [1, 1, 3];

// lower_bound
assert_eq!(a.lower_bound(&0), 0);
assert_eq!(a.lower_bound(&1), 0);
assert_eq!(a.lower_bound(&2), 2);
assert_eq!(a.lower_bound(&3), 2);
assert_eq!(a.lower_bound(&4), 3);

// upper_bound
assert_eq!(a.upper_bound(&0), 0);
assert_eq!(a.upper_bound(&1), 2);
assert_eq!(a.upper_bound(&2), 2);
assert_eq!(a.upper_bound(&3), 3);
assert_eq!(a.upper_bound(&4), 3);
```

```rust
# fn main() {
use std::collections::BTreeSet;

let mut set = BTreeSet::new();
set.insert(3);
set.insert(5);
set.insert(8);

// 4以上を探した時の最初の要素
assert_eq!(Some(&5), set.range(4..).next());

// 7未満を探したときの最後の要素
assert_eq!(Some(&5), set.range(..7).next_back());

// 8以下を探したときの最後の要素
assert_eq!(Some(&8), set.range(..=8).next_back());
# }
```
