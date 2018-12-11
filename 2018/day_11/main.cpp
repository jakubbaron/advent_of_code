#include <iostream>
#include <vector>
#include <iomanip>
#include <algorithm>
#include <numeric>

int main(int argc, char** argv) {
  constexpr auto serial_number = 9110;
  //constexpr auto serial_number = 18;
  constexpr auto size = 300;
  using Grid = std::vector<std::vector<int> >;
  Grid grid;
  for(auto i = 0; i < size; i++) {
    grid.emplace_back(std::vector<int>(size, 0)); 
  }

  //calc values for the grid
  for(int y = 1; y <= size; y++) {
    for(int x = 1; x <= size; x++) {
      auto rack_id = x + 10;
      auto power_level = y * rack_id;
      auto product = power_level + serial_number;
      product *= rack_id;
      const auto str_product = std::to_string(product);
      char digit = '0';
      if(str_product.size() >= 3) {
        digit = str_product.substr(str_product.length() - 3, 1)[0];
      }
      auto cell_power_level = static_cast<int>(digit - '0') - 5;
      grid[y-1][x-1] = cell_power_level;
    }
  }

  int64_t max_val = 0;
  int max_x = 0;
  int max_y = 0;
  int max_size = 0;
  for(int sq_size = 1; sq_size < 300; sq_size++) {
    for(int y = 0; y < size - sq_size; y++) {
      for(int x = 0; x < size - sq_size; x++) {
        int64_t sum = 0;
        for(int dy = 0; dy < sq_size; dy++) {
          for(int dx = 0; dx < sq_size; dx++) {
            sum += grid[y+dy][x+dx];
          }
        }
        if(sum > max_val) {
          max_val = sum;
          max_x = x + 1;
          max_y = y + 1;
          max_size = sq_size;
        }
      }
    }
  }
  std::cout << "Max point X,Y: " << max_x << " " << max_y << " val: " << max_val << 
    " size: " << max_size << std::endl;

 // for(auto row = 10; row < 15; row++) {
 //   for(auto col = 18; col < 25; col++) {
 //     std::cout << std::setw(3) << sum[row][col] << " ";
 //   }
 //   std::cout << std::endl;
 // }

  return EXIT_SUCCESS;

}
