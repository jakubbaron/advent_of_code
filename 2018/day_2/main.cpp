#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <algorithm>

std::pair<bool, bool> char_counts(std::string& in_str) {
  if(in_str.empty()) return std::make_pair(false, false);;
  std::sort(in_str.begin(), in_str.end());
  //std::cout << in_str << "\n";
  auto char_it = in_str.begin();
  char last_char = *char_it;
  bool has_three = false;
  bool has_two = false;
  int count = 0;

  for(; char_it != in_str.end(); char_it++) {
    if(*char_it == last_char) {
      count++;
    } else {
      if(count == 2) {
        has_two = true;
      } else if (count == 3) {
        has_three = true;
      }
      count = 1;
    }

    last_char = *char_it;
  }
  return std::make_pair(has_two, has_three);
}

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  int counter[2] = {0,0};
  while (std::getline(infile, line)) {
    const auto result = char_counts(line);
    if(result.first) counter[0]++;
    if(result.second) counter[1]++;
  }
  std::cout << "Two letters[" << counter[0] << "] Three letters[" << counter[1] << "]\n"; 
  std::cout << "Check sum: " << counter[0] * counter[1] << "\n";
  return 0;
}
