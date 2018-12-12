#include <iostream>
#include <fstream>
#include <string>
#include <vector>
#include <map>

class Plants {
  public:
    Plants(const std::string& line) {
      //m_plants = "....." + line + "............";
      m_plants = "....." + line + "............";
    }
    void print_plants() const {
      std::cout << m_plants << std::endl;
    }
    void add_mask(const std::string& line) {
      static const std::string separator = " => ";
      const std::string mask = line.substr(0, line.find(separator));
      const std::string result = line.substr(line.find(separator) + separator.size());
      m_masks.emplace(std::make_pair(mask, result[0]));
    }

    auto count_plants() const {
      int sum = 0;
      auto first_pos = m_plants.find_first_of('#');
      auto last_pos = m_plants.find_last_of('#');
      std::cout << "First pos: " << first_pos << " last pos: " << last_pos << " ";
      for(auto i = first_pos; i <= last_pos; i++) {
        if(m_plants[i] != '#') {
          continue;
        }
        sum += (i + shift);
      }
      return sum;
    }


    void apply_masks() {
      std::string new_plants_generation(m_plants);
      int i = 0;
      for(auto it = m_plants.begin(); it != m_plants.end() - 3; it++, i++) {
        std::string mask(it, it+5);
        if(!m_masks.count(mask)) {
          //std::cerr << "can't find requested mask![" << mask << "]" << std::endl;
          new_plants_generation[i+2] = '.';
          continue;
        }
        new_plants_generation[i+2] = m_masks[mask];
      }
      m_plants = new_plants_generation + std::string(1, '.');
      std::cout << std::count_if(m_plants.begin(), m_plants.end(), [](char c) { return c == '#';}) << " count: " << count_plants() << " " << std::endl;
    }

  private:
		int shift = -5;
    std::string m_plants;
    std::map<std::string, char> m_masks;
};

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  std::getline(infile, line);
  const std::string sought_for = "initial state: ";
  auto in_state = line.substr(line.find(sought_for) + sought_for.size());
  std::getline(infile, line);

  Plants p{in_state};

  while(std::getline(infile, line)) {
    p.add_mask(line);
  }

  //50000000000
  for(long i = 1; i <= 50000000000; i++) {
    std::cout << i << ": ";
    p.apply_masks();
		//p.print_plants();
  }
  std::cout << "Counted plants: " << p.count_plants() << std::endl;


  return EXIT_SUCCESS;
}
