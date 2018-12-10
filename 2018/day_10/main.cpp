#include <iostream>
#include <fstream>
#include <cstdio>
#include <iomanip>
#include <vector>
#include <map>
#include <algorithm>

class Point {
  public:
    Point(const std::string& line) {
      if (std::sscanf(line.c_str(), "position=< %d, %d> velocity=<%d,  %d>", &x, &y, &velocity_x, &velocity_y) != 4) {
        throw std::invalid_argument(line);
      }
    }

    void move_one_second() {
      x += velocity_x;
      y += velocity_y;
    }

    auto get_x() const {
      return x;
    }

    auto get_y() const {
      return y;
    }
    
    void place_on_plane(char** plane, int plane_size) {
      plane[x][y] = '#';
    }

  private:
    int x;
    int y;
    int velocity_x;
    int velocity_y;
};

void print_plane(char** plane, int plane_size, int second) {
  std::ofstream outfile;
  outfile.open("second" + std::to_string(second) + ".dat");
  for(auto x = 0; x < plane_size; x++) {
    for(auto y = 0; y < plane_size; y++) {
      outfile << std::setw(1) << plane[y][x];
    }
    outfile << std::endl;
  }
  outfile.close();
}

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;

  std::vector<Point> points;

  while (std::getline(infile, line)) {
    points.push_back(Point{line}); 
  }


  for(int second = 1; second < 11000; second++) {
    std::map<int, int> x_frequency;
    std::map<int, int> y_frequency;
    for(auto& point: points) {
      point.move_one_second();
      x_frequency[point.get_x()]++;
      y_frequency[point.get_y()]++;
    }
    auto max_frequency_x = std::max_element(x_frequency.begin(), x_frequency.end(),
            [](const std::pair<int, int>& p1, const std::pair<int, int>& p2) {
            return p1.second < p2.second; });
    auto max_frequency_y = std::max_element(y_frequency.begin(), y_frequency.end(),
            [](const std::pair<int, int>& p1, const std::pair<int, int>& p2) {
            return p1.second < p2.second; });
    if(max_frequency_x->second > 35 || max_frequency_y->second > 35) {
      const auto plane_size = 400;
      char** plane = new char*[plane_size];
      for(auto i = 0; i < plane_size; i++) {
        plane[i] = new char[plane_size]; 
      }

      for(auto i = 0; i < plane_size; i++) {
        for(auto j = 0; j < plane_size; j++) {
          plane[i][j] = '.';
        }
      }

      for(auto& point: points) {
        point.place_on_plane(plane, plane_size);
      } 
      print_plane(plane, plane_size, second);
      for(auto i = 0; i < plane_size; i++) {
        delete[] plane[i];
      }
      delete[] plane;
    }
  }
  return EXIT_SUCCESS;
}
