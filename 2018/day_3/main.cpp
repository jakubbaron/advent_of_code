#include <iostream>
#include <fstream>
#include <cstdio>
#include <map>

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

  void cut_material(int material[][1000], size_t material_size = 1000) const {
    //TODO prob need to check if we don't cut beyond the material
    //like from_left_edge + width < material_size
    //from_top_edge + height < material_size
    for(int i = from_left_edge; i < from_left_edge + width; i++) {
      for(int j = from_top_edge; j < from_top_edge + height; j++) {
        material[i][j]++;
      }
    }
  }
  bool is_not_overlapping(int material[][1000]) const {
    for(int i = from_left_edge; i < from_left_edge + width; i++) {
      for(int j = from_top_edge; j < from_top_edge + height; j++) {
        if(material[i][j] != 1) return false;
      }
    }
    return true;
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
  std::map<int, Cutout> cutouts;
  while (std::getline(infile, line)) {
    Cutout cutout{line};
    cutout.cut_material(material);
    cutouts.emplace(cutout.id, std::move(cutout));
  }
  int double_claimed{0};
  for(int i = 0; i< material_size; i++) {
    for(int j = 0; j< material_size; j++) {
      if(material[i][j] > 1) double_claimed++;
    }
  }
  std::cout << "Double or more claimed: " << double_claimed << std::endl;

  for(const auto& item: cutouts) {
    if(item.second.is_not_overlapping(material)) {
      std::cout << "Cutout ID[" << item.first << "] isn't overlapping with any other" << std::endl; 
    }
  }

  return 0;
}
