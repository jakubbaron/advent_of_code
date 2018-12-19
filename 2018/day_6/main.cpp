#include <iostream>
#include <fstream>
#include <cstdio>
#include <vector>
#include <algorithm>
#include <set>
#include <map>
#include <queue>

constexpr auto arr_size = 360;

struct Coords {
  int X;
  int Y;
};

struct Point {
  Point(char id, int x, int y): ID(id), X(x), Y(y) {
  }
  Point(char id, const std::string& line): ID(id) {
    if(sscanf(line.c_str(), "%d, %d", &X, &Y) != 2) {
      throw new std::invalid_argument("Can't parse[" + line + "] to Poitn's coordinates");
    }
  }

  const char ID;
  int X;
  int Y;

  int distance(const Point& other) {
    return std::abs(X - other.X) + std::abs(Y - other.Y); 
  }
  }
  const std::string to_string() const {
    return std::string(1, ID) + ": " + std::to_string(X) + ", " + std::to_string(Y);
  }
  void put_onto_plane(char plane[][arr_size]) const {
    plane[X][Y] = ID;
  }
  void expand(char plane[][arr_size]) const { 

  }
};

void print_plane(char plane[][arr_size]) {
  for(int i = 0; i < arr_size; i++) {
    for(int j= 0; j < arr_size; j++) {
      std::cout << std::string(1, plane[i][j]);
    }
    std::cout << "\n";
  }
}

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  std::map<char, Point> points;

  char id{'a'};
  while (std::getline(infile, line)) {
    points.emplace(id, Point{id, line}); 
    id++;
    if(id > 'z') {
      id = 'A';
    }
  }
  for(const auto& item: points) {
    std::cout << item.second.to_string() << "\n";
  }

  char plane[arr_size][arr_size];
  for(int i = 0; i < arr_size; i++) {
    for(int j= 0; j < arr_size; j++) {
      plane[i][j] = '0';
    }
  }

  for(const auto& item: points) {
    item.second.put_onto_plane(plane);
  }
  std::cout << std::string(1, points.at('A').crawl_left(plane)) << std::endl;
  return EXIT_SUCCESS;
}
