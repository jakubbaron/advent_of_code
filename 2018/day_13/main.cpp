#include <iostream>
#include <fstream>
#include <memory>
#include <vector>


enum class Direction { CLOCKWISE, COUNTER_CLOCKWISE };
struct Point {
  int X;
  int Y;
};
class Cart {
  public:
    Cart() {

    }

  private:
    Point m_position;
    Direction m_direction;
};

using SCart = std::shared_ptr<Cart>;
class Circuit {
  public:
    Circuit(const size_t length) {

    }
    void add_cart(SCart cart) {
      m_carts.emplace_back(std::move(cart));
    }

    void move_all_carts() {

    }
  private:
    std::vector<SCart> m_carts;
    Point m_position;
    size_t m_length;
    size_t m_height;
};

int main(int argc, char** argv) {
  std::ifstream infile("input.txt");
  std::string line;
  while (std::getline(infile, line)) {

  }

  return EXIT_SUCCESS;
}
