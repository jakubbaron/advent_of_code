#include <iostream>
#include <fstream>
#include <stack>
#include <algorithm>

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  constexpr auto polarity = 'a' - 'A';
  std::getline(infile, line); //no while loop, we expect only one line;
  if(line.empty()) {
    std::cerr << "Empty file" << std::endl;
    return EXIT_FAILURE;
  }
  std::stack<char> polymer;
  for(int i = 0; i < line.size(); i++) {
    char current_char = line[i];
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
  std::cout << "Left out elements: " << polymer.size() << " from: " << line.size() << std::endl;

  return EXIT_SUCCESS;
}
