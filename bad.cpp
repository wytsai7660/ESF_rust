#include <array>
#include <iostream>
#include <numeric>

int main() {
  int n;
  std::cin >> n;
  std::cout << INT_MAX + n << '\n';

  int arr[5]{1, 2, 3, 4, 5};
  size_t idx;
  std::cin >> idx;
  std::cout << arr[idx] << '\n';

  std::array<int, 5> cpp_arr{1, 2, 3, 4, 5};
  std::cin >> idx;
  std::cout << cpp_arr[idx] << '\n';
}
