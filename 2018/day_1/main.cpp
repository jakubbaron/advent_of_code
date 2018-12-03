#include <iostream>
#include <fstream>
#include <sstream>
#include <string>


int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  long frequency{0}, last_frequency{0};
  while (std::getline(infile, line))
  {
    std::istringstream iss(line);
    int change{0};
    if (!(iss >> change)) { 
      std::cerr << "Coudln't get line, finishing";
      break;
    } // error
    last_frequency = frequency;
    frequency += change;
    std::cout << "Current frequency " << last_frequency << ", change of " << change << "; resulting frequency " << frequency << ".\n";
  }
  std::cout << "Result frequency: " << frequency << std::endl;
  return 0;
}
