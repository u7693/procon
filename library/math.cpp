#include <bits/stdc++.h>

// 最大公約数
int gcd(int x, int y) {
  if (x < y)
    std::swap(x, y);
  while (y > 0) {
    int r = x % y;
    x = y;
    y = r;
  }
  return x;
}

// 素数判定
int isprime(int x) {
  if (x == 2)
    return true;
  if (x < 2 || x % 2 == 0)
    return false;

  int i = 3;
  while (i <= std::sqrt(x)) {
    if (x % i == 0)
      return false;
    i += 2;
  }
  return true;
}
