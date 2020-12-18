#include <algorithm>
#include <cassert>
#include <cmath>
#include <fstream>
#include <iostream>
#include <string>
#include <tuple>
#include <unordered_set>
#include <vector>
using namespace std;

// auto read_input(const string& filename) {
//   ifstream ifs(filename.c_str());
//   string s;
//   int y = 0;
//   while (ifs >> s) {
//     for (int x = 0; x < s.size(); ++x) {
//       if (s[x] == '#') {
//         res.emplace(x, y, 0);
//       }
//     }
//     ++y;
//   }
//   return res;
// }

int last[30000001];
int solve(vector<int> input, int last_turn = 2020) {
  fill(last, last + sizeof(last) / sizeof(last[0]), 0);

  for (int i = 0; i < input.size(); i++) last[input[i]] = i + 1;

  // start at turn4
  int next = 0;
  int now = 0;
  for (int turn = input.size() + 1; turn <= last_turn; turn++) {
    now = next;
    // cout << now << endl;
    // seen before
    if (last[now]) {
      next = turn - last[now];
    } else {
      next = 0;
    }
    last[now] = turn;
  }
  return now;
}

int main() {
  //   assert(solve({0, 3, 6}) == 436);
  //   assert(solve({1, 3, 2}) == 1);
  //   assert(solve({2, 1, 3}) == 10);
  //   assert(solve({1, 2, 3}) == 27);
  //   assert(solve({2, 3, 1}) == 78);
  //   assert(solve({3, 2, 1}) == 438);
  //   assert(solve({3, 1, 2}) == 1836);
  //   int ans = solve({0, 1, 4, 13, 15, 12, 16});
  //   cout << ans << endl;

  constexpr int T = 30000000;
  assert(solve({0, 3, 6}, T) == 175594);
  assert(solve({3, 1, 2}, T) == 362);

  int ans = solve({0, 1, 4, 13, 15, 12, 16}, T);
  cout << ans << endl;

  return 0;
}
