#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <set>


int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  long frequency{0}, last_frequency{0};
  std::set<long> frequencies;
  bool found{false};
  while(!found) {
    std::ifstream infile("input.txt");
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
      if(frequencies.count(frequency)) { 
        std::cout << "reached frequency twice: " << frequency << std::endl;
        found = true;
        break;
      } else {
        frequencies.insert(frequency);
      }
      std::cout << "Current frequency " << last_frequency << ", change of " << change << "; resulting frequency " << frequency << ".\n";
    }
  }
  std::cout << "Result frequency: " << frequency << std::endl;
  std::cout << "Set count: " << frequencies.size() << std::endl; 
  return 0;
}
