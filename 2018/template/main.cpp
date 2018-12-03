#include <iostream>
#include <fstream>
#include <sstream>

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  while (std::getline(infile, line)) {
    std::istringstream iss(line);
    int change{0};
    if (!(iss >> change)) { 
      std::cerr << "Coudln't get line, finishing";
      break;
    } // error
  }

  return 0;
}
