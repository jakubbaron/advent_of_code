#include <iostream>
#include <fstream>
#include <stack>
#include <algorithm>
#include <cctype>
#include <set>

auto react_polymer(const std::string& line, char skip_char = '\0') {
  constexpr auto polarity = 'a' - 'A';
  std::stack<char> polymer;
  for(int i = 0; i < line.size(); i++) {
    char current_char = line[i];
    if(std::toupper(current_char) == std::toupper(skip_char)) {
      continue;
    }

    if(polymer.empty()) {
      polymer.push(current_char);
      continue;
    }

    char top_char = polymer.top();

    if(std::abs(top_char - current_char) == polarity) {
      polymer.pop();
    } else {
      polymer.push(current_char);
    }
  }
  return polymer.size();
}

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  std::getline(infile, line); //no while loop, we expect only one line;
  if(line.empty()) {
    std::cerr << "Empty file" << std::endl;
    return EXIT_FAILURE;
  }
  std::cout << "Left out elements: " << react_polymer(line) << " from: " << line.size() << std::endl;

  std::set<size_t> polymer_lengths;
  for(char remove_char = 'a'; remove_char <= 'z'; remove_char++) {
    polymer_lengths.insert(react_polymer(line, remove_char)); 
  }
  std::cout << "The shortest polymer length: " << *polymer_lengths.begin() << std::endl;

  return EXIT_SUCCESS;
}
