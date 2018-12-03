#include <iostream>
#include <fstream>
#include <cstdio>

struct Cutout {
  Cutout(const std::string& line) {
    if (std::sscanf(line.c_str(), "#%d @ %d,%d: %dx%d", &id, &from_left_edge, &from_top_edge, &width, &height) != 5) {
      throw std::invalid_argument("Input in invalid format[" + line + "] cannot create Cutout object");
    }
  }

  int id;
  int from_left_edge;
  int from_top_edge;
  int width;
  int height;

  void cut_material(int material[][1000], size_t material_size = 1000) {
    //TODO prob need to check if we don't cut beyond the material
    //like from_left_edge + width < material_size
    //from_top_edge + height < material_size
    for(int i = from_left_edge; i < from_left_edge + width; i++) {
      for(int j = from_top_edge; j < from_top_edge + height; j++) {
        material[i][j]++;
      }
    }
  }
};

int main(int argc, char** argv) {
  constexpr auto material_size = 1000;
  int material[material_size][material_size];
  for(int i = 0; i< material_size; i++) {
    for(int j = 0; j< material_size; j++) {
      material[i][j] = 0;
    }
  }

  std::ifstream infile("input.txt");
  std::string line;
  while (std::getline(infile, line)) {
    Cutout cutout{line};
    cutout.cut_material(material);
  }
  int double_claimed{0};
  for(int i = 0; i< material_size; i++) {
    for(int j = 0; j< material_size; j++) {
      if(material[i][j] > 1) double_claimed++;
    }
  }
  std::cout << "Double or more claimed: " << double_claimed << std::endl;
  return 0;
}
