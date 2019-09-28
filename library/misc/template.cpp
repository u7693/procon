#include <bits/stdc++.h>
#include <boost/range/adaptor/indexed.hpp>
using namespace std;

#define rep(i, n) for (int i = 0; i < (int)(n); i++)
#define all(x) (x).begin(), (x).end()

template <typename T>
std::ostream &operator<<(std::ostream &ostr, const std::vector<T> &v) {
  for (const auto &i : v | boost::adaptors::indexed())
    ostr << (i.index() > 0 ? " " : "") << i.value();
  return ostr;
}

inline void Main() {
  // code
}

int main() {
  std::cin.tie(0);
  std::ios::sync_with_stdio(false);
  std::cout << std::fixed << std::setprecision(20);
  Main();
  return 0;
}
