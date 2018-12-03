#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <algorithm>
#include <set>
#include <map>
#include <vector>

std::pair<bool, bool> check_two_three_chars(std::string& in_str) {
  if(in_str.empty()) return std::make_pair(false, false);;
  std::sort(in_str.begin(), in_str.end());

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

bool diff_by_one(const std::string& in1, const std::string& in2) {
  if(in1.size() != in2.size()) return false;
  int count_diffs{0};
  for(int i=0; i< in1.size(); i++) {
    if(in1[i] != in2[i]) count_diffs++; 
    if(count_diffs > 1) return false;
  }
  return true;
}

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  int counter[2] = {0,0};
  while (std::getline(infile, line)) {
    const auto result = check_two_three_chars(line);
    if(result.first) counter[0]++;
    if(result.second) counter[1]++;
  }
  std::cout << "Two letters[" << counter[0] << "] Three letters[" << counter[1] << "]\n"; 
  std::cout << "Check sum: " << counter[0] * counter[1] << "\n";
  
  // PART TWO
  infile.clear();
  infile.seekg(0, std::ios::beg);
  
  std::vector<std::string> my_strings;
  while (std::getline(infile, line)) {
    my_strings.emplace_back(std::move(line));
  }
  for(int i=0; i < my_strings.size(); i++) {
    for(int j = i+1; j<my_strings.size(); j++) {
      if(diff_by_one(my_strings[i], my_strings[j])) {
        std::cout << my_strings[i] << " " << my_strings[j] << std::endl;
      }
    }
  }

  return 0;
}
